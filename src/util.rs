use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// return an iterator over all the lines in the file
pub fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
