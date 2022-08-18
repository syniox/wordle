use console;
use std::io::{self, Write};

mod args;
use args::Args;
use clap::Parser;

/// The main function for the tty game
fn main_tty(args: args::Args) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "I am in a tty. Please print {}!",
        console::style("colorful characters").bold().blink().blue()
    );

    print!("{}", console::style("Your name: ").bold().red());
    io::stdout().flush().unwrap();

    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    println!("Welcome to wordle, {}!", line.trim());

    Ok(())
}

/// The main function for the tests
fn main_tst(args: args::Args) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is_tty = atty::is(atty::Stream::Stdout);
    let args = Args::parse();
    println!("{:?}",args); // REMEMBER TO REMOVE

    if is_tty{
        main_tty(args)
    } else {
        main_tst(args)
    }
}
