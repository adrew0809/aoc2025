// Advent of Code 2025 Day 2
// A. Drew

fn main() {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let invalid_sum = BufReader::new(File::open("data/day02/input.txt").unwrap())
        .split(b',')
        .map(|u| std::str::from_utf8(&u.unwrap()).unwrap().to_owned())
        .map(|input| parse_range(&input).unwrap())
        .flat_map(|[a, b]| {
            let a = next_invalid_half(a);
            let b = prev_invalid_half(b);
            (a..=b).map(from_half)
        })
        .sum::<u64>();
    println!("sum of invalid ids: {}", invalid_sum);
}

fn parse_range(input: &str) -> Result<[u64; 2], nom::Err<nom::error::Error<&str>>> {
    use nom::{
        character::complete::{char, digit1, multispace0},
        combinator::all_consuming,
        sequence::{separated_pair, terminated},
        Parser,
    };
    all_consuming(terminated(
        separated_pair(
            digit1.map_res(str::parse::<u64>),
            char('-'),
            digit1.map_res(str::parse::<u64>),
        ),
        multispace0,
    ))
    .map(|(a, b)| [a, b])
    .parse(input)
    .map(|(_, x)| x)
}

fn split(d: u64) -> [u64; 2] {
    let n = d.ilog10() + 1;
    let m = 10_u64.pow((n + 1) / 2);
    [d / m, d % m]
}

fn even_digits_floor(d: u64) -> u64 {
    let n = num_digits(d);
    if n % 2 == 0 {
        d
    } else {
        10u64.pow(n - 1) - 1
    }
}

fn even_digits_ceil(d: u64) -> u64 {
    let n = num_digits(d);
    if n % 2 == 0 {
        d
    } else {
        10u64.pow(n)
    }
}

fn num_digits(d: u64) -> u32 {
    d.ilog10() + 1
}

fn next_invalid_half(d: u64) -> u64 {
    let d = even_digits_ceil(d);
    let [a, b] = split(d);
    if a < b {
        a + 1
    } else {
        a
    }
}

fn prev_invalid_half(d: u64) -> u64 {
    let d = even_digits_floor(d);
    let [a, b] = split(d);
    if a <= b {
        a
    } else {
        a - 1
    }
}

fn from_half(d: u64) -> u64 {
    let n = num_digits(d);
    d * 10u64.pow(n) + d
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufRead;

    const CONTENT: &str = "\
11-22,\
95-115,\
998-1012,\
1188511880-1188511890,\
222220-222224,\
1698522-1698528,\
446443-446449,\
38593856-38593862,\
565653-565659,\
824824821-824824827,\
2121212118-2121212124
";

    const RANGES: [[u64; 2]; 11] = [
        [11, 22],
        [95, 115],
        [998, 1012],
        [1188511880, 1188511890],
        [222220, 222224],
        [1698522, 1698528],
        [446443, 446449],
        [38593856, 38593862],
        [565653, 565659],
        [824824821, 824824827],
        [2121212118, 2121212124],
    ];

    #[test]
    fn test_parse_id_ranges() {
        let buff = std::io::Cursor::new(CONTENT);
        let ranges: Vec<_> = buff
            .split(b',')
            .map(|u| std::str::from_utf8(&u.unwrap()).unwrap().to_owned())
            .map(|input| parse_range(&input).unwrap())
            .collect();
        assert_eq!(ranges, RANGES);
    }

    #[test]
    fn test_split() {
        assert_eq!(split(123451), [123, 451]);
        assert_eq!(split(10012345), [1001, 2345]);
        assert_eq!(split(10), [1, 0]);
        assert_eq!(split(310), [3, 10]);
        assert_eq!(split(54321), [54, 321]);
    }

    #[test]
    fn test_even_digits_floor_even() {
        assert_eq!(even_digits_floor(10), 10);
        assert_eq!(even_digits_floor(1234), 1234);
        assert_eq!(even_digits_floor(991234), 991234);
    }

    #[test]
    fn test_even_digits_floor_odd() {
        assert_eq!(even_digits_floor(101), 99);
        assert_eq!(even_digits_floor(12345), 9999);
        assert_eq!(even_digits_floor(1000000), 999999);
    }

    #[test]
    fn test_even_digits_ceil_even() {
        assert_eq!(even_digits_ceil(10), 10);
        assert_eq!(even_digits_ceil(1234), 1234);
        assert_eq!(even_digits_ceil(991234), 991234);
    }

    #[test]
    fn test_even_digits_ceil_odd() {
        assert_eq!(even_digits_ceil(101), 1000);
        assert_eq!(even_digits_ceil(12345), 100000);
        assert_eq!(even_digits_ceil(1000000), 10000000);
    }

    #[test]
    fn test_next_invalid_half_even() {
        assert_eq!(next_invalid_half(123451), 124);
        assert_eq!(next_invalid_half(10012345), 1002);
        assert_eq!(next_invalid_half(10), 1);
        assert_eq!(next_invalid_half(909001), 909);
        assert_eq!(next_invalid_half(1188511880), 11885);
        assert_eq!(next_invalid_half(11), 1);
    }

    #[test]
    fn test_next_invalid_half_odd() {
        assert_eq!(next_invalid_half(1), 1);
        assert_eq!(next_invalid_half(310), 10);
        assert_eq!(next_invalid_half(54321), 100);
    }

    #[test]
    fn test_prev_invalid_half_event() {
        assert_eq!(prev_invalid_half(1310), 12);
        assert_eq!(prev_invalid_half(1013), 10);
        assert_eq!(prev_invalid_half(1013), 10);
        assert_eq!(prev_invalid_half(1188511890), 11885);
        assert_eq!(prev_invalid_half(22), 2);
    }

    #[test]
    fn test_prev_invalid_half_odd() {
        assert_eq!(prev_invalid_half(310), 9);
        assert_eq!(prev_invalid_half(103), 9);
        assert_eq!(prev_invalid_half(113), 9);
        assert_eq!(prev_invalid_half(188511890), 9999);
    }

    #[test]
    fn test_sample() {
        let invalid_sums: Vec<u64> = RANGES
            .iter()
            .map(|[a, b]| {
                let a = next_invalid_half(*a);
                let b = prev_invalid_half(*b);
                (a..=b).map(from_half).sum()
            })
            .collect();
        assert_eq!(
            invalid_sums,
            [33, 99, 1010, 1188511885, 222222, 0, 446446, 38593859, 0, 0, 0]
        );
        assert_eq!(invalid_sums.iter().sum::<u64>(), 1227775554);
    }
}
