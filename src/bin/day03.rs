// Advent of Code 2025 Day 3
// A. Drew

fn main() {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Error, ErrorKind};

    let batteries = 12;

    let total_output: Result<u64, _> = File::open("data/day03/input.txt").and_then(|file| {
        BufReader::new(file)
            .lines()
            .map(|line| {
                line.and_then(|l| {
                    parse_bank(&l)
                        .and_then(|b| largest_joltage(batteries, &b))
                        .ok_or(Error::from(ErrorKind::InvalidInput))
                })
            })
            .sum()
    });
    println!("{:?}", total_output);
}

// Parse a string of digits into a vector of integers
fn parse_bank(line: &str) -> Option<Vec<u64>> {
    line.chars()
        .map(|c| c.to_digit(10).map(u64::from))
        .collect()
}

// Find the largest joltage using the provided number of batteries in the bank
fn largest_joltage(batteries: u32, bank: &[u64]) -> Option<u64> {
    (0..batteries)
        .rev()
        .try_fold((0, 0), |(joltage, begin), rank| {
            find_max_battery(begin, rank, bank).map(|(a, i)| (joltage + a * 10u64.pow(rank), i + 1))
        })
        .map(|(joltage, _)| joltage)
}

// Finds the value and index of the maximum battery while making sure there are at least
// rank more batteries to the right
fn find_max_battery(begin: usize, rank: u32, bank: &[u64]) -> Option<(u64, usize)> {
    let n = bank.len();
    bank[begin..n - rank as usize]
        .iter()
        .enumerate()
        .rev()
        .max_by(|(_, x), (_, y)| x.cmp(y))
        .map(|(i, a)| (*a, begin + i))
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

    const BANKS: [[u64; 15]; 4] = [
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
    fn test_sample_with_2() {
        let joltages: Vec<_> = BANKS.iter().map(|bank| largest_joltage(2, bank)).collect();
        assert_eq!(joltages, [Some(98), Some(89), Some(78), Some(92)]);
    }

    #[test]
    fn test_sample_with_12() {
        let joltages: Vec<_> = BANKS.iter().map(|bank| largest_joltage(12, bank)).collect();
        assert_eq!(
            joltages,
            [
                Some(987654321111),
                Some(811111111119),
                Some(434234234278),
                Some(888911112111)
            ]
        );
    }
}
