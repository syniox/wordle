use console;
use rand;
use rand::{SeedableRng, prelude::SliceRandom};
use std::collections::HashSet;
use std::{
    io::{self, Write},
};

mod args;
use args::Args;
use clap::Parser;

mod utils;

mod game;
use game::{Game, Stats};

mod builtin_words;

// return final_words_list, final_words, valid_words
fn load_word_list(args: &args::Args) -> (Vec<String>, HashSet<String>, HashSet<String>){
    let mut final_words_list: Vec<String> = match args.final_set.as_ref() {
        None => utils::from_arr(builtin_words::FINAL),
        Some(f) => utils::arr_from_file(f)
    };
    final_words_list.iter_mut().for_each(|x| x.make_ascii_uppercase());
    let final_words: HashSet<String> = final_words_list.iter().map(|x| x.clone()).collect();
    let mut valid_words_list: Vec<String> = match args.acceptable_set.as_ref() {
        None => utils::from_arr(builtin_words::ACCEPTABLE),
        Some(f) => utils::arr_from_file(f)
    };
    valid_words_list.iter_mut().for_each(|x| x.make_ascii_uppercase());
    let valid_words: HashSet<String> = valid_words_list.into_iter().collect();
    for word in final_words_list.iter() {
        assert!(valid_words.contains(word));
    }
    if args.random {
        let mut rng = rand::rngs::StdRng::seed_from_u64(args.seed.unwrap());
        final_words_list.shuffle(&mut rng);
    }
    (final_words_list, final_words, valid_words)
}

fn word_from_tty(args: &Args, game: &Game, words: Option<&HashSet<String>>) -> String {
    loop {
        match utils::read_word(words){
            Ok(w) => if !args.difficult || game.hard_check(&w) { break w },
            Err(e) => println!("{}, please type a correct word with 5 characters.",e)
        };
    }
}

/// The main function for the tty game
fn main_tty(args: Args) -> Result<(), utils::ErrorT> {
    let (final_words_list, final_words, valid_words) = load_word_list(&args);
    let mut stats = match args.state.as_ref() {
        None => Stats::new(),
        Some(f) => serde_json::from_str(&utils::str_from_file(f))?
    };

    println!("{}", console::style("Welcome to wordle!").blink().blue());

    for day in args.day.unwrap() - 1.. {
        let mut game = Game::new();
        let answer = if let Some(w) = args.word.as_ref() {
            w.clone()
        } else if !args.random {
            //TODO check whether the word is valid
            println!("{}", console::style(
                "You aren't using random mode. Please type answer first.").red());
            word_from_tty(&args, &game, Some(&final_words))
        } else {
            final_words_list[day as usize].to_string()
        };
        game.set_answer(answer);

        let mut win = false;
        for round in 0..utils::ROUNDS {
            let word = word_from_tty(&args, &game, Some(&valid_words));
            win = game.guess(word.clone());
            // output colorized results
            let (col_pos, col_alpha) = game.show_col();
            for (i, c) in word.chars().enumerate() {
                print!("{}", utils::colorize_id(col_pos[i]).apply_to(c));
            }
            print!(" ");
            for(i, c) in ('A'..='Z').enumerate() {
                print!("{}", utils::colorize_id(col_alpha[i]).apply_to(c));
            }
            println!("");
            io::stdout().flush()?;
            if win {
                println!("Congratulations! You made it with {} {}.",
                    round+1, if round == 0 { "guess" } else { "guesses" });
                break;
            }
        }
        if !win {
            println!("Sorry, but the correct answer is {}", game.show_answer());
        }
        if args.stats {

        }
        if args.word.is_none() {

        }
    }
    if let Some(file) = args.state {
        utils::str_to_file(serde_json::to_string_pretty(&stats)?.as_str(), &file);
    }
    Ok(())
}

/// The main function for the tests
fn main_tst(args: args::Args) -> Result<(), utils::ErrorT> {
    let (final_words_list, final_words, valid_words) = load_word_list(&args);
    let mut stats = match args.state.as_ref() {
        None => Stats::new(),
        Some(f) => serde_json::from_str(&utils::str_from_file(f))?
    };

    for day in args.day.unwrap() - 1.. {
        let mut game = Game::new();
        let answer = if let Some(w) = args.word.as_ref() {
            w.clone()
        } else if !args.random {
            //TODO check whether the word is valid
            utils::read_word(Some(&final_words))?
        } else {
            final_words_list[day as usize].to_string()
        };
        game.set_answer(answer);

        let mut win = false;
        for round in 0..utils::ROUNDS {
            let word = loop {
                if let Ok(word) = utils::read_word(Some(&valid_words)){
                    if !args.difficult || game.hard_check(&word) {
                        break word;
                    }
                }
                println!("INVALID");
            };
            win = game.guess(word);
            println!("{}", game);
            if win {
                println!("CORRECT {}", round+1);
                break;
            }
        }
        if !win {
            println!("FAILED {}", game.show_answer());
        }
        if args.stats {
            stats.store_game(game);
            stats.print_stats();
        }
        if args.word.is_none() {
            let line = utils::read_line()?;
            assert!(line == "N" || line == "Y" || line == "");
            if line == "N" {
                break;
            }
        } else {
            break;
        }
    }
    if let Some(file) = args.state {
        utils::str_to_file(serde_json::to_string_pretty(&stats)?.as_str(), &file);
    }
    Ok(())
}

/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), utils::ErrorT> {
    let is_tty = atty::is(atty::Stream::Stdout);
    let mut args = Args::parse();
    args.refine();

    if is_tty{
        main_tty(args)
    } else {
        main_tst(args)
    }
}
