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
    #[clap(short, long, value_parser)]
    pub day: Option<i32>,

    /// Specify random seed
    #[clap(short, long, value_parser)]
    pub seed: Option<u64>,

    /// Specify final set
    #[clap(short, long = "final-set", value_parser)]
    pub fset: Option<String>,

    /// Specify acceptable set
    #[clap(short, long = "acceptable-set", value_parser)]
    pub aset: Option<String>,

    /// store and load game state using file <state>
    #[clap(short = 'S', long, value_parser)]
    pub state: Option<String>,

    /// Specify config file
    #[clap(short, long, value_parser)]
    pub config: Option<String>
}

impl Args{
    pub fn refine(&mut self) {
        if self.seed.is_some() || self.day.is_some() {
            self.random = true;
        }
        self.day = self.day.or(Some(1));
        self.seed = self.seed.or(Some(0));
        if self.random && self.word.is_some() {
            panic!("-w cannot be used in random mode");
        }
    }
}