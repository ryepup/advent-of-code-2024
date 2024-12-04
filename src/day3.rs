use std::{
    fmt,
    fs::File,
    io::{self, Read},
    path::Path,
};

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
            ('m', _) => Self::M,
            ('u', Self::M) => Self::U,
            ('l', Self::U) => Self::L,
            ('(', Self::L) => Self::OpenParen,
            (x, Self::OpenParen) if char::is_ascii_digit(&x) => Self::FirstNumber(x.to_string()),
            (x, Self::FirstNumber(s)) if char::is_ascii_digit(&x) => {
                let mut s = s.to_owned();
                s.push(x);
                Self::FirstNumber(s)
            }
            (',', Self::FirstNumber(s)) => match s.parse::<i32>() {
                Ok(a) => Self::Comma(a),
                _ => Self::Empty,
            },
            (x, Self::Comma(n)) if char::is_ascii_digit(&x) => {
                Self::SecondNumber(*n, x.to_string())
            }
            (x, Self::SecondNumber(a, s)) if char::is_ascii_digit(&x) => {
                let mut s = s.to_owned();
                s.push(x);
                Self::SecondNumber(*a, s)
            }
            (')', Self::SecondNumber(a, s)) => match s.parse::<i32>() {
                Ok(b) => Self::CloseParen(a * b),
                _ => Self::Empty,
            },
            _ => Self::Empty,
        }
    }
}

pub fn solve1(filename: impl AsRef<Path>) -> Result<i32> {
    let mut state = ParserState::Empty;
    let mut total = 0;

    for b in chars(filename)? {
        state = match state.next(b) {
            ParserState::CloseParen(n) => {
                total += n;
                ParserState::Empty
            }
            s => s,
        }
    }
    Ok(total)
}

enum EnableParserState {
    Empty,
    D,
    O,
    N,
    Apostrophe,
    T,
    OpenParen(bool),
    CloseParen(bool),
}

impl EnableParserState {
    fn next(&self, c: char) -> Self {
        match (c, self) {
            ('d', Self::Empty) => Self::D,
            ('o', Self::D) => Self::O,
            ('(', Self::O) => Self::OpenParen(true),
            (')', Self::OpenParen(e)) => Self::CloseParen(*e),
            ('n', Self::O) => Self::N,
            ('\'', Self::N) => Self::Apostrophe,
            ('t', Self::Apostrophe) => Self::T,
            ('(', Self::T) => Self::OpenParen(false),
            _ => Self::Empty,
        }
    }
}

pub fn solve2(filename: impl AsRef<Path>) -> Result<i32> {
    let mut mul_parser = ParserState::Empty;
    let mut enable_parser = EnableParserState::Empty;

    let mut total = 0;
    let mut enabled = true;

    for b in chars(filename)? {
        if enabled {
            mul_parser = match mul_parser.next(b) {
                ParserState::CloseParen(n) => {
                    total += n;
                    ParserState::Empty
                }
                s => s,
            }
        }
        enable_parser = match enable_parser.next(b) {
            EnableParserState::CloseParen(b) => {
                enabled = b;
                EnableParserState::Empty
            }
            s => s,
        }
    }
    Ok(total)
}

fn chars(filename: impl AsRef<Path>) -> Result<impl Iterator<Item = char>> {
    if let Ok(file) = File::open(filename) {
        let iter = io::BufReader::new(file)
            .bytes()
            .flatten()
            .map(|b| char::from_u32(b.into()))
            .flatten();
        Ok(iter)
    } else {
        Err(Day3Error)
    }
}
