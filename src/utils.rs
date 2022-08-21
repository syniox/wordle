use std::{io, fs};
use std::collections::HashSet;

pub const ROUNDS: i32 = 6;

pub type ErrorT = Box<dyn std::error::Error>;

pub fn apmax<T: Copy + std::cmp::Ord>(a: &mut T, b: T) {
    if *a < b { *a = b; }
}

pub fn read_line() -> Result<String, ErrorT> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    line.truncate(line.trim().len());
    Ok(line)
}

pub fn read_word(words: Option<&HashSet<String>>) -> Result<String, ErrorT> {
    let mut line = read_line()?;
    if line.len() != 5 {
        return Err(ErrorT::from(format!("invalid input length: {}", line)));
    }
    line.make_ascii_lowercase();
    if let Some(w) = words.as_ref() {
        if w.contains(&line) {
            Ok(line)
        } else {
            Err(ErrorT::from(format!("invalid input word")))
        }
    } else{
        Ok(line)
    }
}

// pub fn arr2set(arr: &[&str]) -> HashSet<String> {
pub fn from_arr<T: std::iter::FromIterator<String>>(arr: &[&str]) -> T {
    arr.iter().map(|x| x.to_string()).collect()
}

pub fn read_from_file<T: std::iter::FromIterator<String>>(file: &str) -> T {
    let content = fs::read_to_string(file)
        .expect(format!("cannot read file {}", file).as_str());
    content.split_whitespace().map(|x| x.to_string()).collect()
}