use console;
use rand::{SeedableRng, prelude::SliceRandom};
use std::{
    io::{self, Write},
    collections::HashSet
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

fn read_word_hinted(args: &Args, game: &Game, words: Option<&HashSet<String>>) -> String {
    loop {
        match utils::read_word(words){
            Ok(w) => if !args.difficult || game.hard_check(&w) {
                break w
            } else {
                if args.tty {
                    utils::warn("Please type a word according to the information you've got.");
                } else { println!("INVALID") }
            },
            Err(e) => if args.tty {
                utils::warn(&format!("{}, please type a correct 5-character word.",e));
            } else { println!("INVALID") }
        };
    }
}


/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), utils::ErrorT> {
    let mut args = Args::parse();
    args.tty = atty::is(atty::Stream::Stdout);
    args.refine();
    let args = args;

    let (final_words_list, final_words, valid_words) = load_word_list(&args);
    let mut stats = match args.state.as_ref() {
        None => Stats::new(),
        Some(f) => serde_json::from_str(&utils::str_from_file(f))?
    };
    if args.tty {
        println!("Welcome to {}!", console::style("wordle").blink().blue());
    }

    for day in args.day.unwrap() - 1.. {
        // Init game
        let mut game = Game::new();
        let answer = if let Some(w) = args.word.as_ref() {
            w.clone()
        } else if !args.random {
            //TODO check whether the word is valid
            if args.tty {
                utils::warn("You aren't using random mode. Please type answer first.");
                read_word_hinted(&args, &game, Some(&final_words))
            } else {
                utils::read_word(Some(&final_words))?
            }
        } else {
            final_words_list[day as usize].to_string()
        };
        game.set_answer(answer);
        if args.tty {
            println!("Now, please guess the 5-character word!");
        }
        let mut win = false;
        for round in 0..utils::ROUNDS {
            let word = read_word_hinted(&args, &game, Some(&valid_words));
            win = game.guess(word.clone());
            // print guess result
            if args.tty {
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
            } else {
                println!("{}", game);
            }

            if win {
                if args.tty {
                    println!("Congratulations! You made it with {} {}.",
                        round+1, if round == 0 { "guess" } else { "guesses" });
                } else {
                    println!("CORRECT {}", round+1);
                }
                break;
            }
        }
        if !win {
            if args.tty {
                println!("Sorry that you failed. The answer is {}", game.show_answer());
            } else{
                println!("FAILED {}", game.show_answer());
            }
        }
        if args.stats {
            stats.store_game(game);
            stats.print_stats(args.tty);
        }
        // find out whether the program should continue
        if args.word.is_none() {
            let mut line = utils::read_line()?;
            while args.tty && line != "N" && line != "Y" && line != "" {
                line = utils::read_line()?;
            }
            let line = line;
            if line != "N" && line != "Y" && line != "" {
                panic!("should we continue?");
            }
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
