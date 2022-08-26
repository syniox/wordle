use serde::{Deserialize, Serialize};
use std::{
    cmp,
    collections::{HashMap, HashSet},
    fmt,
    iter::zip,
};

use crate::{utils, utils::apmax};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
struct State {
    answer: String,
    guesses: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Stats {
    #[serde(default)]
    total_rounds: i32,
    #[serde(default)]
    games: Vec<State>,
}

#[derive(Clone)]
pub struct Game {
    state: State,
    // 3: G, 2: Y, 1: R, 0: X
    // len26, stores each alpha's color
    col_alpha: Vec<i8>,
    // len5, stores each position's color of latest guess
    col_pos: Vec<i8>,
    // len26, stores how much times an alpha should be used at least
    lim_alpha: Vec<i8>,
}

impl State {
    pub fn new() -> State {
        State {
            answer: String::new(),
            guesses: Vec::<String>::new(),
        }
    }
}

impl Stats {
    fn stat_cmp((s1, i1): (&str, &i32), (s2, i2): (&str, &i32)) -> cmp::Ordering {
        //TODO: check cmp for String
        if *i1 == *i2 {
            s2.cmp(s1)
        } else {
            i1.cmp(i2)
        }
    }

    pub fn new() -> Stats {
        Stats {
            total_rounds: 0,
            games: vec![],
        }
    }
    pub fn store_game(&mut self, game: Game) {
        self.total_rounds += 1;
        self.games.push(game.state);
    }
    // Calculate win_rounds, lose_rounds, avg_guesses
    pub fn feed_stats(&self) -> (i32, i32, f64) {
        let (win_rounds, win_guesses) = self
            .games
            .iter()
            .filter(|x| Some(&x.answer) == x.guesses.last())
            .map(|x| (1, x.guesses.len()))
            .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
        let lose_rounds = self.total_rounds - win_rounds;
        let avg_guesses = if win_rounds == 0 {
            0f64
        } else {
            win_guesses as f64 / win_rounds as f64
        };
        (win_rounds, lose_rounds, avg_guesses)
    }
    // Find words that used most
    pub fn feed_words(&self) -> Vec<(&str, i32)> {
        let mut map = HashMap::<&str, i32>::new();
        // load stats into helper vaiables
        for game in self.games.iter() {
            for guess in game.guesses.iter() {
                map.entry(guess.as_str())
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
            }
        }

        let mut w_list: Vec<(&str, i32)> = map.into_iter().collect();
        w_list.sort_by(|(s1, i1), (s2, i2)| Self::stat_cmp((s1, i1), (s2, i2)));
        w_list.reverse();
        w_list.truncate(5);
        w_list
    }

    pub fn print_stats(&self, is_tty: bool) {
        let (win_rounds, lose_rounds, avg_guesses) = self.feed_stats();
        let w_list = self.feed_words();

        if is_tty {
            let win_colored = console::style(format!("Win: {}", win_rounds)).green();
            let lose_colored = console::style(format!("Lose: {}", lose_rounds)).red();
            println!(
                "{}, {}, Avg guesses: {:.2}",
                win_colored, lose_colored, avg_guesses
            );
            println!("Used most:");
            for (i, w) in w_list.iter().enumerate() {
                if i != 0 {
                    print!(" ");
                }
                print!("{} ({} time(s))", w.0, w.1);
            }
        } else {
            println!("{} {} {:.2}", win_rounds, lose_rounds, avg_guesses);
            for (i, w) in w_list.iter().enumerate() {
                if i != 0 {
                    print!(" ");
                }
                print!("{} {}", w.0, w.1);
            }
        }
        println!("");
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            Self::vec2str(&self.col_pos),
            Self::vec2str(&self.col_alpha)
        )
    }
}

impl Game {
    fn alpha2id(c: char) -> usize {
        c as usize - 'A' as usize
    }
    fn color2id(c: char) -> i8 {
        match c {
            'G' => 3,
            'Y' => 2,
            'R' => 1,
            'X' => 0,
            _ => panic!("Unknown color: {}", c),
        }
    }
    fn id2color(id: i8) -> char {
        match id {
            3 => 'G',
            2 => 'Y',
            1 => 'R',
            0 => 'X',
            _ => panic!("unknown color id {}", id),
        }
    }

    pub fn new() -> Game {
        Game {
            state: State::new(),
            col_alpha: vec![0i8; utils::ALPHAS],
            col_pos: vec![0i8; utils::LEN],
            lim_alpha: vec![0i8; utils::ALPHAS],
        }
    }

    pub fn won(&self) -> bool {
        match self.state.guesses.len() {
            0 => false,
            l => self.state.guesses[l - 1] == self.state.answer,
        }
    }
    pub fn ended(&self) -> bool {
        let stat = &self.state;
        match stat.guesses.len() {
            utils::ROUNDS => true,
            _ => self.won(),
        }
    }

    pub fn rounds(&self) -> usize {
        self.state.guesses.len()
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
    pub fn show_col(&self) -> (&Vec<i8>, &Vec<i8>) {
        (&self.col_pos, &self.col_alpha)
    }
    // Check whether the word meets the requirement of hard mode
    pub fn hard_check(&self, guess: &str) -> bool {
        let mut cnt_alpha = vec![0i8; utils::ALPHAS];
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
                return false;
            }
        }
        true
    }
    // Receive user's guesses and update color/requirement
    pub fn guess(&mut self, guess: String) -> bool {
        assert!(guess.len() == utils::LEN);
        self.state.guesses.push(guess.clone());
        let answer = self.state.answer.clone();
        let mut cnt_alpha = vec![0i8; utils::ALPHAS];
        for ca in answer.chars() {
            cnt_alpha[Self::alpha2id(ca)] += 1;
        }
        let req_alpha = cnt_alpha.clone();
        // Color good position to Green
        for (i, (ca, cg)) in zip(answer.chars(), guess.chars()).enumerate() {
            if ca == cg {
                let alpha_id = Self::alpha2id(ca);
                let color_id = Self::color2id('G');
                cnt_alpha[alpha_id] -= 1;
                self.col_pos[i] = color_id;
                self.col_alpha[alpha_id] = color_id;
            }
        }
        // Color other position
        for (i, (ca, cg)) in zip(answer.chars(), guess.chars()).enumerate() {
            let alpha_id = Self::alpha2id(cg);
            if ca != cg {
                cnt_alpha[alpha_id] -= 1;
                let color_id = Self::color2id(if cnt_alpha[alpha_id] >= 0 { 'Y' } else { 'R' });
                self.col_pos[i] = color_id;
                apmax(&mut self.col_alpha[alpha_id], color_id);
            }
        }
        // Calc how many times should an alpha be used at least
        for a in 0..self.lim_alpha.len() {
            apmax(
                &mut self.lim_alpha[a],
                req_alpha[a] - cmp::max(0i8, cnt_alpha[a]),
            );
        }
        guess == self.state.answer
    }

    // Find words that may still be answer in the current word list
    pub fn find_valid(&self, words: Vec<String>) -> Vec<String> {
        assert!(!self.state.guesses.is_empty());
        let mut has_red = vec![false; utils::ALPHAS];
        let mut ulim_alpha = vec![0i8; utils::ALPHAS];
        let guess = self.state.guesses.last().unwrap();
        for (i, c) in guess.chars().enumerate() {
            if self.col_pos[i] >= Self::color2id('Y') {
                ulim_alpha[Self::alpha2id(c)] += 1;
            }
            if self.col_pos[i] == Self::color2id('R') {
                has_red[Self::alpha2id(c)] = true;
            }
        }
        let ulim_alpha: Vec<i8> = has_red.iter()
            .zip(ulim_alpha.iter())
            .map(|(&red, &lim)| if red { lim } else { 100i8 })
            .collect();

        words.into_iter()
            .filter(|word| {
                let mut cnt_alpha = [0i8; utils::ALPHAS];
                let mut invld = false;
                // invalid green position
                invld |= self.state.answer.chars()
                    .zip(word.chars())
                    .zip(self.col_pos.iter())
                    .any(|((ans, ch), &col)| col == 3 && ans != ch);
                // invalid yellow position
                invld |= guess.chars()
                    .zip(word.chars())
                    .zip(self.col_pos.iter())
                    .any(|((guess, word), &col)| col == 2 && guess == word);
                // invalid char count
                word.chars()
                    .for_each(|c| cnt_alpha[Self::alpha2id(c) as usize] += 1);
                for i in 0..utils::ALPHAS {
                    if cnt_alpha[i] > ulim_alpha[i] || cnt_alpha[i] < self.lim_alpha[i] {
                        invld |= true;
                    }
                }
                !invld
            })
            .collect()
    }
}
