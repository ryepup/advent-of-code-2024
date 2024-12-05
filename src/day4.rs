use std::{path::Path, usize};

use crate::util;

#[derive(PartialEq)]
enum Value {
    Unknown,
    X,
    M,
    A,
    S,
}

impl From<char> for Value {
    fn from(value: char) -> Self {
        match value {
            'X' => Self::X,
            'M' => Self::M,
            'A' => Self::A,
            'S' => Self::S,
            _ => Self::Unknown,
        }
    }
}

struct Square {
    row: usize,
    col: usize,
    value: Value,
}

struct Puzzle(Vec<String>);

impl Puzzle {
    fn scan(&self) -> PuzzleIterator {
        PuzzleIterator {
            puzzle: &self,
            row: 0,
            col: 0,
        }
    }

    fn at(&self, row: usize, col: usize) -> Option<Square> {
        self.0
            .iter()
            .nth(row)
            .and_then(|line| line.chars().nth(col))
            .map(|c| Square {
                row,
                col,
                value: c.into(),
            })
    }

    /// count how many times we can find X-M-A-S starting from this position
    fn count_xmas(&self, sq: Square) -> i32 {
        if sq.value != Value::X {
            return 0;
        }

        let rest = vec![Value::M, Value::A, Value::S];
        let mut found = 0;

        for col_delta in vec![-1, 0, 1] {
            for row_delta in vec![-1, 0, 1] {
                if row_delta == 0 && col_delta == 0 {
                    continue;
                }

                if rest.iter().enumerate().all(|(i, val)| {
                    let offset: i32 = (i + 1).try_into().expect("i should fit in i32");

                    // type inference couldn't handle adding on the same expression as try_into
                    let mut row: i32 = sq.row.try_into().expect("row should fit in i32");
                    row += offset * row_delta;
                    let mut col: i32 = sq.col.try_into().expect("col should fit in i32");
                    col += offset * col_delta;

                    if row < 0 || col < 0 {
                        return false;
                    } else {
                        let r = row.try_into().expect("already checked for negatives");
                        let c = col.try_into().expect("already checked for negatives");
                        match self.at(r, c) {
                            Some(s) if s.value == *val => true,
                            _ => false,
                        }
                    }
                }) {
                    found += 1
                }
            }
        }

        found
    }
}

struct PuzzleIterator<'a> {
    puzzle: &'a Puzzle,
    row: usize,
    col: usize,
}

impl Iterator for PuzzleIterator<'_> {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(sq) = self.puzzle.at(self.row, self.col) {
            self.col += 1;
            Some(sq)
        } else {
            // try the next line
            self.col = 0;
            self.row += 1;
            if let Some(sq) = self.puzzle.at(self.row, self.col) {
                self.col += 1;
                Some(sq)
            } else {
                None
            }
        }
    }
}

pub fn solve1(filename: impl AsRef<Path>) -> i32 {
    let puzzle = Puzzle(
        util::read_lines(filename)
            .expect("could not read")
            .flatten()
            .collect(),
    );

    puzzle.scan().map(|s| puzzle.count_xmas(s)).sum()
}
