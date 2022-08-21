use std::{fmt, cmp, iter::zip};
use serde::Serialize;

use crate::{
    utils::apmax,
    args::Args
};

#[derive(Serialize)]
struct State {
    answer: String,
    guesses: Vec<String>
}

pub struct Game {
    state: State,
    // 3: G, 2: Y, 1: R, 0: X
    // len26, stores each alpha's color
    col_alpha: Vec<i8>, 
    // len5, stores each position's color of latest guess
    col_pos: Vec<i8>,
    // len26, stores how much times an alpha should be used at least
    lim_alpha: Vec<i8> 
}

impl State {
    pub fn new() -> State {
        State { answer: String::new(), guesses: Vec::<String>::new() }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", Self::vec2str(&self.col_pos), Self::vec2str(&self.col_alpha))
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
        Game {
            state: State::new(),
            col_alpha: vec![0i8; 26],
            col_pos: vec![0i8; 5],
            lim_alpha: vec![0i8; 26]
        }
    }
    pub fn set_answer(&mut self, answer: String) {
        self.state.answer = answer;
    }
    pub fn vec2str(v: &Vec<i8>) -> String {
        v.iter().map(|x| Self::id2color(*x)).collect()
    }
    pub fn show_answer(&self) -> &str {
        &self.state.answer
    }

    pub fn hard_check(&self, guess: &str) -> bool {
        let mut cnt_alpha = vec![0i8; 26];
        // ensure user uses all green state
        for (i, (ca, cg)) in zip(self.state.answer.chars(), guess.chars()).enumerate() {
            let col = Self::id2color(self.col_pos[i]);
            if col == 'G' && ca != cg {
                return false;
            }
            cnt_alpha[Self::alpha2id(cg)] += 1;
        }
        // ensure user uses all yellow state
        for i in 0..cnt_alpha.len() {
            if self.lim_alpha[i] > cnt_alpha[i] {
                return false
            }
        }
        true
    }

    pub fn guess(&mut self, guess: String) -> bool {
        assert!(guess.len() == 5);
        self.state.guesses.push(guess.clone());
        let answer = self.state.answer.clone();
        let mut cnt_alpha = vec![0i8; 26];
        for ca in answer.chars() {
            cnt_alpha[Self::alpha2id(ca)] += 1;
        }
        let req_alpha = cnt_alpha.clone();
        // color good position to G
        for (i, (ca, cg)) in zip(answer.chars(), guess.chars()).enumerate() {
            if ca == cg {
                let alpha_id = Self::alpha2id(ca);
                let color_id = Self::color2id('G');
                cnt_alpha[alpha_id] -= 1;
                self.col_pos[i] = color_id;
                self.col_alpha[alpha_id] = color_id;
            }
        }
        // color other position
        for (i, (ca, cg)) in zip(answer.chars(), guess.chars()).enumerate() {
            let alpha_id = Self::alpha2id(cg);
            cnt_alpha[alpha_id] -= 1;
            if ca != cg {
                let color_id = Self::color2id(
                    if cnt_alpha[alpha_id] >= 0 { 'Y' } else { 'R' }
                    );
                self.col_pos[i] = color_id;
                apmax(&mut self.col_alpha[alpha_id], color_id);
            }
        }
        // calc how many times should an alpha be used at least
        for a in 0..self.lim_alpha.len() {
            apmax(&mut self.lim_alpha[a], req_alpha[a] - cmp::max(0i8, cnt_alpha[a]));
        }
        guess == self.state.answer
    }
}
