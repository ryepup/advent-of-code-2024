use std::{
    fmt,
    fs::File,
    io::{self, Read},
    path::Path,
};

use crate::util;
// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone, PartialEq)]
pub struct Day3Error;

impl fmt::Display for Day3Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something went wrong!")
    }
}

pub type Result<T> = std::result::Result<T, Day3Error>;

enum ParserState {
    Empty,
    M,
    U,
    L,
    OpenParen,
    FirstNumber(String),
    Comma(i32),
    SecondNumber(i32, String),
    CloseParen(i32),
}

impl ParserState {
    fn next(&self, c: char) -> Self {
        match (c, self) {
            ('m', _) => ParserState::M,
            ('u', ParserState::M) => ParserState::U,
            ('l', ParserState::U) => ParserState::L,
            ('(', ParserState::L) => ParserState::OpenParen,
            (x, ParserState::OpenParen) if char::is_ascii_digit(&x) => {
                ParserState::FirstNumber(x.to_string())
            }
            (x, ParserState::FirstNumber(s)) if char::is_ascii_digit(&x) => {
                let mut i_dont_understand_refs = s.clone();
                i_dont_understand_refs.push(x);
                ParserState::FirstNumber(i_dont_understand_refs)
            }
            (',', ParserState::FirstNumber(s)) => match s.parse::<i32>() {
                Ok(a) => ParserState::Comma(a),
                _ => ParserState::Empty,
            },
            (x, ParserState::Comma(n)) if char::is_ascii_digit(&x) => {
                ParserState::SecondNumber(*n, x.to_string())
            }
            (x, ParserState::SecondNumber(a, s)) if char::is_ascii_digit(&x) => {
                let mut i_dont_understand_refs = s.clone();
                i_dont_understand_refs.push(x);
                ParserState::SecondNumber(*a, i_dont_understand_refs)
            }
            (')', ParserState::SecondNumber(a, s)) => match s.parse::<i32>() {
                Ok(b) => ParserState::CloseParen(a * b),
                _ => ParserState::Empty,
            },
            _ => ParserState::Empty,
        }
    }
}

pub fn solve1(filename: impl AsRef<Path>) -> Result<i32> {
    if let Ok(file) = File::open(filename) {
        let mut state = ParserState::Empty;
        let mut total = 0;
        let chars = io::BufReader::new(file)
            .bytes()
            .flatten()
            .map(|b| char::from_u32(b.into()))
            .flatten();
        for b in chars {
            state = match state.next(b) {
                ParserState::CloseParen(n) => {
                    total += n;
                    ParserState::Empty
                }
                s => s,
            }
        }
        Ok(total)
    } else {
        Err(Day3Error)
    }
}

pub fn solve2(filename: impl AsRef<Path>) -> Result<usize> {
    Ok(0)
}
