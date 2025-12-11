// Advent of Code 2025 Day 2
// A. Drew

fn main() {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let ranges: Vec<_> = BufReader::new(File::open("data/day02/input.txt").unwrap())
        .split(b',')
        .map(|u| std::str::from_utf8(&u.unwrap()).unwrap().to_owned())
        .map(|input| parse_range(&input).unwrap())
        .collect();
    let invalid_sum = ranges
        .clone()
        .into_iter()
        .flat_map(find_repeated_twice)
        .sum::<u64>();
    println!("sum of invalid ids: {}", invalid_sum);
    let invalid_sum = ranges.into_iter().flat_map(find_repeated_any).sum::<u64>();
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

fn repeats_every(sequence: &[u8], n: usize) -> bool {
    let mut iter = sequence.chunks(n);
    iter.next().map_or(false, |head| iter.all(|x| head == x))
}

fn find_repeated_twice([a, b]: [u64; 2]) -> Vec<u64> {
    (a..=b)
        .filter(|x| {
            let x = Vec::<u8>::from(x.to_string());
            let n = x.len();
            n > 1 && repeats_every(&x, (n + 1) / 2)
        })
        .collect()
}

fn find_repeated_any([a, b]: [u64; 2]) -> Vec<u64> {
    (a..=b)
        .filter(|x| {
            let x = Vec::<u8>::from(x.to_string());
            let n = x.len();
            (1..=n / 2).any(|i| repeats_every(&x, i))
        })
        .collect()
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
    fn test_repeates_every() {
        assert!(repeats_every(&[1, 2, 3, 1, 2, 3], 3));
        assert!(!repeats_every(&[1, 2, 3, 1, 2, 3, 1], 3));
        assert!(repeats_every(&[1, 4, 4, 1, 4, 4, 1, 4, 4], 3));
    }

    #[test]
    fn test_sample_part_1() {
        let invalid_ids: Vec<_> = RANGES.into_iter().map(find_repeated_twice).collect();
        assert_eq!(
            invalid_ids,
            [
                vec![11, 22],
                vec![99],
                vec![1010],
                vec![1188511885],
                vec![222222],
                vec![],
                vec![446446],
                vec![38593859],
                vec![],
                vec![],
                vec![]
            ]
        );
    }

    #[test]
    fn test_sample_part_2() {
        let invalid_ids: Vec<_> = RANGES.into_iter().map(find_repeated_any).collect();
        assert_eq!(
            invalid_ids,
            [
                vec![11, 22],
                vec![99, 111],
                vec![999, 1010],
                vec![1188511885],
                vec![222222],
                vec![],
                vec![446446],
                vec![38593859],
                vec![565656],
                vec![824824824],
                vec![2121212121]
            ]
        );
    }
}
