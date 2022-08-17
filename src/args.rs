use clap::Parser;

#[derive(Parser,Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args{
    /// Specify the word to guess
    #[clap(short, long, value_parser)]
    word: Option<String>,

    /// Enable random mode
    #[clap(short, long, value_parser)]
    random: bool,

    /// Enable hard mode
    #[clap(short = 'D', long = "difficult", value_parser)]
    hard: bool,

    /// Print statistic after every term
    #[clap(short = 't', long, value_parser)]
    stat: bool,

    /// Specify starting day
    #[clap(short, long, value_parser, default_value_t = 1)]
    day: i32,

    /// Specify random seed
    #[clap(short, long, value_parser, default_value_t = 0)]
    seed: i32, // TODO i32?

    /// Specify final set
    #[clap(short, long = "final-set", value_parser)]
    fset: Option<String>,

    /// Specify acceptable set
    #[clap(short, long = "acceptable-set", value_parser)]
    aset: Option<String>,

    /// Store game state
    #[clap(short = 'S', long, value_parser)]
    state: bool,

    /// Specify config file
    #[clap(short, long, value_parser)]
    config: Option<String>
}
