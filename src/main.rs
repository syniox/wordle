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
    let mut final_words_list: Vec<String> = match args.fset.as_ref() {
        None => utils::from_arr(builtin_words::FINAL),
        Some(f) => utils::arr_from_file(f)
    };
    final_words_list.iter_mut().for_each(|x| x.make_ascii_uppercase());
    let final_words: HashSet<String> = final_words_list.iter().map(|x| x.clone()).collect();
    let mut valid_words_list: Vec<String> = match args.aset.as_ref() {
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

/// The main function for the tty game
fn main_tty(args: args::Args) -> Result<(), utils::ErrorT> {
    println!(
        "I am in a tty. Please print {}!",
        console::style("colorful characters").bold().blink().blue()
    );

    print!("{}", console::style("Your name: ").bold().red());
    io::stdout().flush().unwrap();

    let line = utils::read_line()?;
    println!("Welcome to wordle, {}!", line.trim());

    Ok(())
}

/// The main function for the tests
fn main_tst(args: args::Args) -> Result<(), utils::ErrorT> {
    let (final_words_list, final_words, valid_words) = load_word_list(&args);
    let mut stats = match args.state.as_ref() {
        None => Stats::new(),
        Some(f) => serde_json::from_str(&utils::str_from_file(f))?
    };

    for day in args.day.unwrap()-1.. {
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
                    if !args.hard || game.hard_check(&word) {
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
            println!("FAILED {}", game.show_answer().to_ascii_uppercase());
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
    //let is_tty = atty::is(atty::Stream::Stdout);
    let mut args = Args::parse();
    //println!("{:?}",args);
    args.refine();

    main_tst(args)
    /*if is_tty{
        main_tty(args)
    } else {
        main_tst(args)
    }*/
}
