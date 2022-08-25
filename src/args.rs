use crate::utils::str_from_file;
use clap::Parser;
use serde::Deserialize;

#[derive(Default, Parser, Debug, Deserialize)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// show acceptable word list
    #[clap(short, long, value_parser)]
    #[serde(skip)]
    pub hint: bool,

    /// whether using tty
    /// should not be parsed by serde
    #[clap(skip)]
    #[serde(default)]
    pub tty: bool,

    /// Specify the word to guess
    #[clap(short, long, value_parser)]
    #[serde(default)]
    pub word: Option<String>,

    /// Enable random mode
    #[clap(short, long, value_parser)]
    #[serde(default)]
    pub random: bool,

    /// Enable hard mode
    #[clap(short = 'D', long = "difficult", value_parser)]
    #[serde(default)]
    pub difficult: bool,

    /// Print statistic after every term
    #[clap(short = 't', long, value_parser)]
    #[serde(default)]
    pub stats: bool,

    /// Specify starting day
    #[clap(short, long, value_parser)]
    #[serde(default)]
    pub day: Option<i32>,

    /// Specify random seed
    #[clap(short, long, value_parser)]
    #[serde(default)]
    pub seed: Option<u64>,

    /// Specify final set
    #[clap(short, long = "final-set", value_parser)]
    #[serde(default)]
    pub final_set: Option<String>,

    /// Specify acceptable set
    #[clap(short, long = "acceptable-set", value_parser)]
    #[serde(default)]
    pub acceptable_set: Option<String>,

    /// store and load game state using file <state>
    #[clap(short = 'S', long, value_parser)]
    #[serde(default)]
    pub state: Option<String>,

    /// Specify config file
    #[clap(short, long, value_parser)]
    #[serde(default)]
    pub config: Option<String>,
}

impl Args {
    pub fn refine(&mut self) {
        // port config file into config
        if let Some(cfg) = self.config.as_ref() {
            let alt_arg: Args = serde_json::from_str(&str_from_file(&cfg)).unwrap();
            self.word = self.word.take().or(alt_arg.word);
            self.random |= alt_arg.random;
            self.difficult |= alt_arg.difficult;
            self.stats |= alt_arg.stats;
            self.day = self.day.or(alt_arg.day);
            self.seed = self.seed.or(alt_arg.seed);
            self.final_set = self.final_set.take().or(alt_arg.final_set);
            self.acceptable_set = self.acceptable_set.take().or(alt_arg.acceptable_set);
            self.state = self.state.take().or(alt_arg.state);
            self.config = self.config.take().or(alt_arg.config);
        }
        // random mode check
        if let Some(w) = self.word.as_ref() {
            self.word = Some(w.to_ascii_uppercase());
        }
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
