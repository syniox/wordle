use clap::Parser;

#[derive(Parser,Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args{
    /// Specify the word to guess
    #[clap(short, long, value_parser)]
    pub word: Option<String>,

    /// Enable random mode
    #[clap(short, long, value_parser)]
    pub random: bool,

    /// Enable hard mode
    #[clap(short = 'D', long = "difficult", value_parser)]
    pub hard: bool,

    /// Print statistic after every term
    #[clap(short = 't', long, value_parser)]
    pub stats: bool,

    /// Specify starting day
    #[clap(short, long, value_parser, default_value_t = 1)]
    pub day: i32,

    /// Specify random seed
    #[clap(short, long, value_parser, default_value_t = 0)]
    pub seed: i32, // TODO i32?

    /// Specify final set
    #[clap(short, long = "final-set", value_parser)]
    pub fset: Option<String>,

    /// Specify acceptable set
    #[clap(short, long = "acceptable-set", value_parser)]
    pub aset: Option<String>,

    /// Store game state
    #[clap(short = 'S', long, value_parser)]
    pub state: bool,

    /// Specify config file
    #[clap(short, long, value_parser)]
    pub config: Option<String>
}
