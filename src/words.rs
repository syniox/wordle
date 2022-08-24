use std::collections::HashSet;
use rand::{SeedableRng, prelude::SliceRandom};
use crate::{
    args, utils, builtin_words
};

pub struct Words{
    pub final_list: Vec<String>,
    pub r#final: HashSet<String>,
    pub valid: HashSet<String>
}

impl Words{
    pub fn new(args: &args::Args) -> Words {
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
        Words { final_list: final_words_list, r#final: final_words, valid: valid_words }
    }
}