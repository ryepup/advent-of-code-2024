use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn solve1<P>(filename: P) -> i32
where
    P: AsRef<Path>,
{
    let mut vecA: Vec<i32> = Vec::new();
    let mut vecB: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let array: Vec<i32> = line
                .split("   ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            assert_eq!(array.len(), 2, "should have the same len");

            let pos = vecA.binary_search(&array[0]).unwrap_or_else(|e| e);
            vecA.insert(pos, array[0]);

            let pos = vecB.binary_search(&array[1]).unwrap_or_else(|e| e);
            vecB.insert(pos, array[1]);
        }

        let mut total: i32 = 0;
        for (i, a) in vecA.iter().enumerate() {
            let b = vecB[i];

            total += (b - a).abs();
        }

        return total;
    }
    panic!("how do I return errors?")
}

pub fn solve2<P>(filename: P) -> i32
where
    P: AsRef<Path>,
{
    let mut vecA: Vec<i32> = Vec::new();
    let mut freqB: HashMap<i32, i32> = HashMap::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let array: Vec<i32> = line
                .split("   ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            assert_eq!(array.len(), 2, "should have the same len");

            let pos = vecA.binary_search(&array[0]).unwrap_or_else(|e| e);
            vecA.insert(pos, array[0]);

            match freqB.get(&array[1]) {
                Some(&count) => freqB.insert(array[1], count + 1),
                _ => freqB.insert(array[1], 1),
            };
        }

        let mut total: i32 = 0;
        for a in vecA.iter() {
            match freqB.get(a) {
                Some(&count) => total += a * count,
                None => { /* nothing to do */ }
            };
        }

        return total;
    }
    panic!("how do I return errors?")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_testing() {
        assert_eq!(1, 1);
    }
}
