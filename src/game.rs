use std::{cmp,iter::zip};
use serde::Serialize;

use crate::{
    utils,
    args::Args
};

#[derive(Serialize)]
struct State {
    answer: String,
    guesses: Vec<String>
}

pub struct Game {
    state: State,
    color: Vec<i8> // 3: G, 2: Y, 1: R, 0: X
}

impl State {
    pub fn new() -> State {
        State { answer: String::new(), guesses: Vec::<String>::new() }
    }
}

impl Game{
    fn alpha2id(c: char) -> usize {
        c as usize - 'a' as usize
    }
    fn color2id(c: char) -> i8 {
        match c {
            'G' => 3, 'Y' => 2, 'R' => 1, 'X' => 0,
            _ => panic!("Unknown color: {}", c)
        }
    }
    fn id2color(id: i8) -> char {
        match id {
            3 => 'G', 2 => 'Y', 1 => 'R', 0 => 'X',
            _ => panic!("unknown color id {}", id)
        }
    }

    pub fn new() -> Game {
        Game { state: State::new(), color: vec![0i8; 26] }
    }
    pub fn set_answer(&mut self, answer: String) {
        self.state.answer = answer;
    }
    pub fn vec2str(v: &Vec<i8>) -> String {
        v.iter().map(|x| Self::id2color(*x)).collect()
    }
    pub fn list_color(&self) -> String {
        Self::vec2str(&self.color)
    }
    pub fn show_answer(&self) -> &str {
        &self.state.answer
    }

    pub fn guess(&mut self, guess: String) -> (bool, String) {
        assert!(guess.len() == 5);
        self.state.guesses.push(guess.clone());
        let answer = self.state.answer.clone();
        let mut alpha_cnt = vec![0i8; 26];
        let mut word_color = vec![0i8; 5];
        for ca in answer.chars() {
            alpha_cnt[Self::alpha2id(ca)] += 1;
        }
        // color good position to G
        for (i, (ca, cg)) in zip(answer.chars(), guess.chars()).enumerate() {
            if ca == cg {
                let alpha_id = Self::alpha2id(ca);
                let color_id = Self::color2id('G');
                alpha_cnt[alpha_id] -= 1;
                word_color[i] = color_id;
                self.color[alpha_id] = cmp::max(self.color[alpha_id], color_id);
            }
        }
        // color other position
        for (i, (ca, cg)) in zip(answer.chars(), guess.chars()).enumerate() {
            let alpha_id = Self::alpha2id(cg);
            alpha_cnt[alpha_id] -= 1;
            if ca != cg {
                let color_id = Self::color2id(
                    if alpha_cnt[alpha_id] >= 0 { 'Y' } else { 'R' }
                    );
                word_color[i] = cmp::max(word_color[i], color_id);
                self.color[alpha_id] = cmp::max(self.color[alpha_id], color_id);
            }
        }
        (guess == self.state.answer, Self::vec2str(&word_color))
    }
}
