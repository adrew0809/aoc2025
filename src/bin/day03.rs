// Advent of Code 2025 Day 3
// A. Drew

fn main() {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Error, ErrorKind};

    let total_output: Result<u32, _> = File::open("data/day03/input.txt").and_then(|file| {
        BufReader::new(file)
            .lines()
            .map(|line| {
                line.and_then(|l| {
                    parse_bank(&l)
                        .and_then(|b| largest_joltage(&b))
                        .ok_or(Error::from(ErrorKind::InvalidInput))
                })
            })
            .sum()
    });
    println!("{:?}", total_output);
}

fn parse_bank(line: &str) -> Option<Vec<u32>> {
    line.chars().map(|c| c.to_digit(10)).collect()
}

fn largest_joltage(bank: &[u32]) -> Option<u32> {
    let n = bank.len();
    bank[0..n - 1]
        .iter()
        .enumerate()
        .rev()
        .max_by(|(_, x), (_, y)| x.cmp(y))
        .and_then(|(i, a)| bank[i + 1..].iter().max().map(|b| 10 * a + b))
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";

    const BANKS: [[u32; 15]; 4] = [
        [9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
        [8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
        [2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
        [8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
    ];

    #[test]
    fn test_parsing() {
        let banks: Option<Vec<_>> = CONTENT.lines().map(|line| parse_bank(&line)).collect();
        assert_eq!(banks.unwrap(), BANKS);
    }

    #[test]
    fn test_sample() {
        let js: Vec<_> = BANKS.iter().map(|bank| largest_joltage(bank)).collect();
        assert_eq!(js, [Some(98), Some(89), Some(78), Some(92)]);
    }
}
