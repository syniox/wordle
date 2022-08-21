use std::io;
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
            Err(ErrorT::from(format!("invalid input word: {}", line)))
        }
    } else{
        Ok(line)
    }
}

pub fn arr2set(arr: &[&str]) -> HashSet<String> {
    arr.iter().map(|x| x.to_string()).collect()
}
