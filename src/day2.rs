use std::{fmt, path::Path};

use crate::util;
// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone, PartialEq)]
pub struct Day2Error;

impl fmt::Display for Day2Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something went wrong!")
    }
}

pub type Result<T> = std::result::Result<T, Day2Error>;

pub fn solve1<P>(filename: P) -> Result<usize>
where
    P: AsRef<Path>,
{
    parse(filename).and_then(|reports| Ok(reports.iter().filter(|r| r.safe()).count()))
}

pub fn solve2<P>(filename: P) -> Result<usize>
where
    P: AsRef<Path>,
{
    parse(filename)
        .and_then(|reports| Ok(reports.iter().filter(|r| r.safe_with_dampener()).count()))
}

fn parse<P: AsRef<Path>>(filename: P) -> Result<Vec<Report>> {
    util::read_lines(filename)
        .map_err(|_| Day2Error)?
        .flatten()
        .map(parse_line)
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn safe(&self) -> bool {
        let mut prev = self.levels[0];
        let mut is_increasing: Option<bool> = None;
        for curr in self.levels.iter().skip(1) {
            match (curr - prev, is_increasing) {
                (0, _) => return false,
                (d, _) if d.abs() > 3 => return false,
                (d, Some(true)) if d < 0 => return false,
                (d, Some(false)) if d > 0 => return false,
                (d, None) => is_increasing = Some(d > 0),
                (_, _) => (), // OK
            };

            prev = *curr
        }
        true
    }

    fn safe_with_dampener(&self) -> bool {
        if self.safe() {
            return true;
        }

        // see if we can get a safe set by removing one level

        for i in 0..self.levels.len() {
            let r = Report {
                levels: self
                    .levels
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, v)| if idx == i { None } else { Some(*v) })
                    .collect(),
            };
            if r.safe() {
                return true;
            }
        }
        return false;
    }
}

fn parse_line<S: AsRef<str>>(line: S) -> Result<Report> {
    let levels: std::result::Result<Vec<_>, _> =
        line.as_ref().split(" ").map(|s| s.parse::<i32>()).collect();
    match levels {
        Ok(l) => Ok(Report { levels: l }),
        Err(_) => Err(Day2Error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            Ok(Report {
                levels: vec![1, 2, 7, 8, 9]
            }),
            parse_line("1 2 7 8 9")
        )
    }

    #[test]
    fn test_safe() -> Result<()> {
        assert_eq!(true, parse_line("7 6 4 2 1")?.safe());
        assert_eq!(false, parse_line("1 2 7 8 9")?.safe());
        assert_eq!(false, parse_line("9 7 6 2 1")?.safe());
        assert_eq!(false, parse_line("1 3 2 4 5")?.safe());
        assert_eq!(false, parse_line("8 6 4 4 1")?.safe());
        assert_eq!(true, parse_line("1 3 6 7 9")?.safe());
        Ok(())
    }

    #[test]
    fn test_safe_with_dampener() -> Result<()> {
        assert_eq!(true, parse_line("7 6 4 2 1")?.safe_with_dampener());
        assert_eq!(false, parse_line("1 2 7 8 9")?.safe_with_dampener());
        assert_eq!(false, parse_line("9 7 6 2 1")?.safe_with_dampener());
        assert_eq!(true, parse_line("1 3 2 4 5")?.safe_with_dampener());
        assert_eq!(true, parse_line("8 6 4 4 1")?.safe_with_dampener());
        assert_eq!(true, parse_line("1 3 6 7 9")?.safe_with_dampener());
        Ok(())
    }
}
