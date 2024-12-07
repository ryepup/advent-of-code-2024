use std::{path::Path, usize};

use crate::util;

#[derive(PartialEq, Debug)]
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

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
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

    /// walk starts at a square and then attempts to apply the directions to
    /// move around the puzzle. Returns None if a move goes off the board.
    fn walk(&self, sq: &Square, dirs: impl Iterator<Item = Direction>) -> Option<Square> {
        dirs.fold(Some((sq.row, sq.col)), |pos, d| match (pos, d) {
            (Some((r, c)), Direction::Down) => r.checked_add(1).map(|r| (r, c)),
            (Some((r, c)), Direction::Up) => r.checked_sub(1).map(|r| (r, c)),
            (Some((r, c)), Direction::Left) => c.checked_sub(1).map(|c| (r, c)),
            (Some((r, c)), Direction::Right) => c.checked_add(1).map(|c| (r, c)),
            (None, _) => None,
        })
        .and_then(|(r, c)| self.at(r, c))
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

    /// count how many times we can find M-A-S in Xs starting from this position
    fn count_x_mas(&self, sq: &Square) -> i32 {
        if sq.value != Value::A {
            return 0;
        }

        // upper left and lower right need to be M,S
        // lower left and upper right need to be M,S
        /*
         M S
          A
         M S
        */
        let res: Vec<_> = [
            [Direction::Up, Direction::Left],
            [Direction::Down, Direction::Right],
            [Direction::Up, Direction::Right],
            [Direction::Down, Direction::Left],
        ]
        .iter()
        .map(|dirs| self.walk(sq, dirs.iter().map(|d| *d)))
        .map(|r| r.map(|s| s.value))
        .flatten()
        .collect();

        if res.len() != 4 {
            return 0;
        }

        match (&res[0], &res[1], &res[2], &res[3]) {
            (Value::M, Value::S, Value::M, Value::S) => 1,
            (Value::S, Value::M, Value::M, Value::S) => 1,
            (Value::M, Value::S, Value::S, Value::M) => 1,
            (Value::S, Value::M, Value::S, Value::M) => 1,
            _ => 0,
        }
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

pub fn solve2(filename: impl AsRef<Path>) -> i32 {
    let puzzle = Puzzle(
        util::read_lines(filename)
            .expect("could not read")
            .flatten()
            .collect(),
    );

    puzzle.scan().map(|s| puzzle.count_x_mas(&s)).sum()
}
