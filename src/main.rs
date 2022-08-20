use console;
use std::{
    io::{self, Write},
};

mod args;
use args::Args;
use clap::Parser;

mod utils;

mod game;
use game::Game;

mod builtin_words;

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
    let valid_words = if args.aset.is_none() {
        utils::arr2set(builtin_words::ACCEPTABLE)
    } else {
        unimplemented!();
    };
    let final_words = if args.fset.is_none() {
        utils::arr2set(builtin_words::FINAL)
    } else {
        unimplemented!();
    };

    let mut game = Game::new();
    let answer = utils::read_word(Some(&final_words))?;
    game.set_answer(answer);

    for round in 0..utils::ROUNDS {
        let word = loop {
            if let Ok(word) = utils::read_word(Some(&valid_words)){
                break word
            } else {
                println!("INVALID");
            }
        };
        let (win, word_color) = game.guess(word);
        println!("{} {}", word_color, game.list_color());
        if win {
            println!("CORRECT {}", round+1);
            return Ok(());
        }
    }
    println!("FAILED {}", game.show_answer().to_ascii_uppercase());
    Ok(())
}

/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), utils::ErrorT> {
    let is_tty = atty::is(atty::Stream::Stdout);
    let args = Args::parse();
    //println!("{:?}",args);
    //args.refine();

    if is_tty{
        main_tty(args)
    } else {
        main_tst(args)
    }
}
