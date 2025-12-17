// Advent of Code 2025 Day 5
// A. Drew

use nom::{
    character::complete::{char, digit1, newline},
    combinator::map_res,
    multi::{count, separated_list1},
    sequence::separated_pair,
    Parser,
};
use std::ops::RangeInclusive;

fn main() {
    let answer0 = std::fs::read_to_string("data/day05/input.txt")
        .and_then(|text| {
            parse_database(&text)
                .map_err(|e| std::io::Error::other(e.to_owned()))
                .map(|(_, r)| r)
        })
        .map(|(fresh, ingredients)| fresh_ingredients(&fresh, &ingredients).len());
    println!("{:?}", answer0);
}

fn parse_frash_range(input: &str) -> nom::IResult<&str, RangeInclusive<u64>> {
    separated_pair(number, char('-'), number)
        .map(|(a, b)| a..=b)
        .parse(input)
}

fn number(input: &str) -> nom::IResult<&str, u64> {
    map_res(digit1, str::parse).parse(input)
}

fn parse_fresh_ranges(input: &str) -> nom::IResult<&str, Vec<RangeInclusive<u64>>> {
    separated_list1(newline, parse_frash_range).parse(input)
}

fn parse_ingredients(input: &str) -> nom::IResult<&str, Vec<u64>> {
    separated_list1(newline, number).parse(input)
}

fn parse_database(input: &str) -> nom::IResult<&str, (Vec<RangeInclusive<u64>>, Vec<u64>)> {
    separated_pair(parse_fresh_ranges, count(newline, 2), parse_ingredients).parse(input)
}

fn is_fresh(fresh: &[RangeInclusive<u64>], ingredient: u64) -> bool {
    fresh.iter().any(|range| range.contains(&ingredient))
}

fn fresh_ingredients(fresh: &[RangeInclusive<u64>], ingredients: &[u64]) -> Vec<u64> {
    ingredients
        .iter()
        .filter(|x| is_fresh(fresh, **x))
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXT: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    const FRESH: [RangeInclusive<u64>; 4] = [(3..=5), (10..=14), (16..=20), (12..=18)];

    const IDS: [u64; 6] = [1, 5, 8, 11, 17, 32];

    #[test]
    fn test_ranges() {
        let fresh: Vec<_> = IDS
            .iter()
            .filter(|x| is_fresh(&FRESH, **x))
            .copied()
            .collect();
        assert_eq!(fresh, [5, 11, 17]);
    }

    #[test]
    fn test_parsing() {
        let (_, (ranges, ingredients)) = parse_database(TEXT).unwrap();
        assert_eq!(ranges, FRESH);
        assert_eq!(ingredients, IDS);
    }
}
