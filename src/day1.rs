use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Error};
use std::num::ParseIntError;
use std::path::Path;
use std::{error, fmt};

pub fn solve1<P>(filename: P) -> Result<i32, ParseError>
where
    P: AsRef<Path>,
{
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for (a, b) in parse(filename)? {
        let pos = left_list.binary_search(&a).unwrap_or_else(|e| e);
        left_list.insert(pos, a);

        let pos = right_list.binary_search(&b).unwrap_or_else(|e| e);
        right_list.insert(pos, b)
    }

    let total = left_list
        .iter()
        .zip(right_list)
        .fold(0, |acc, (a, b)| acc + (b - a).abs());
    Ok(total)
}

pub fn solve2<P>(filename: P) -> Result<i32, ParseError>
where
    P: AsRef<Path>,
{
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_histogram: HashMap<i32, i32> = HashMap::new();

    for (a, b) in parse(filename)? {
        let pos = left_list.binary_search(&a).unwrap_or_else(|e| e);
        left_list.insert(pos, a);

        right_histogram.insert(b, 1 + right_histogram.get(&b).copied().unwrap_or_default());
    }

    let total = left_list
        .iter()
        .map(|x| x * right_histogram.get(x).copied().unwrap_or_default())
        .sum();
    Ok(total)
}

/// custom error wrapping everything that might fail.
///
/// feels like a more verbose version of java checked exceptions...
#[derive(Debug)]
pub enum ParseError {
    Read(Error),
    Parse(ParseIntError),
}

// print prettily
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::Read(..) => write!(f, "data file could not be read"),
            ParseError::Parse(..) => write!(f, "the provided string could not be parsed as int"),
        }
    }
}

// magic to unwrap the error if needed
impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ParseError::Read(ref e) => Some(e),
            ParseError::Parse(ref e) => Some(e),
        }
    }
}

// convert from our base errors into out custom error
impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> ParseError {
        ParseError::Parse(err)
    }
}

impl From<Error> for ParseError {
    fn from(err: Error) -> ParseError {
        ParseError::Read(err)
    }
}

/// parse the file into tuples representing each list
fn parse<P>(filename: P) -> Result<Vec<(i32, i32)>, ParseError>
where
    P: AsRef<Path>,
{
    match read_lines(filename) {
        Ok(lines) => match lines.flatten().map(parse_line).collect() {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        },
        Err(e) => Err(e.into()),
    }
}

/// parse a line into a tuple of integers
///
/// this `S: AsRef<str>` jazz allows calling this function with `str` _or_
/// `String`. Rust seems to use generics as a mechanism for overloading.
fn parse_line<S: AsRef<str>>(line: S) -> Result<(i32, i32), ParseIntError> {
    let parts: Result<Vec<_>, _> = line
        .as_ref()
        .split("   ")
        .map(|s| s.parse::<i32>())
        .collect();
    match parts {
        Ok(v) => {
            assert_eq!(v.len(), 2);
            Ok((v[0], v[1]))
        }
        Err(e) => Err(e),
    }
}

// return an iterator over all the lines in the file
fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        assert_eq!(
            vec!["1234", "5678"],
            "1234   5678".split("   ").collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(Ok((1234, 5678)), parse_line("1234   5678"))
    }
}
