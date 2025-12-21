// Advent of Code 2025 Day 6
// A. Drew

use nom::{
    character::complete::{char, digit1, newline, space0, space1},
    combinator::value,
    error::ParseError,
    multi::{many1, separated_list1},
    sequence::{delimited, terminated},
    IResult, Parser,
};
use std::ops::{Add, Mul};

type Op = fn(u64, u64) -> u64;

fn main() {
    let answers: Result<u64, _> = std::fs::read_to_string("data/day06/input.txt")
        .and_then(|text| {
            parse_problems(&text)
                .map_err(|e| std::io::Error::other(e.to_owned()))
                .map(|(_, r)| r)
        })
        .map(|(matrix, ops)| apply(&ops, &matrix).iter().sum());
    println!("{:?}", answers);
}

fn apply(ops: &[Op], matrix: &[u64]) -> Vec<u64> {
    ops.iter()
        .enumerate()
        .map(|(i, op)| {
            matrix
                .iter()
                .skip(i)
                .step_by(ops.len())
                .copied()
                .reduce(op)
                .expect("at least one row of input")
        })
        .collect()
}

fn parse_problems(text: &str) -> IResult<&str, (Vec<u64>, Vec<Op>)> {
    many1(row_of(number_parser))
        .map(|v| v.into_iter().flatten().collect())
        .and(row_of(op_parser))
        .parse(text)
}

fn op_parser(input: &str) -> IResult<&str, Op> {
    value(u64::add as Op, char('+'))
        .or(value(u64::mul as Op, char('*')))
        .parse(input)
}

fn number_parser(input: &str) -> IResult<&str, u64> {
    digit1.map_res(str::parse).parse(input)
}

fn row_of<'a, O, E: ParseError<&'a str>, F>(
    inner: F,
) -> impl Parser<&'a str, Output = Vec<O>, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    terminated(
        delimited(space0, separated_list1(space1, inner), space0),
        newline,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXT: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    const MATRIX: [u64; 3 * 4] = [123, 328, 51, 64, 45, 64, 387, 23, 6, 98, 215, 314];

    const OPS: [Op; 4] = [u64::mul, u64::add, u64::mul, u64::add];

    #[test]
    fn test_apply() {
        let ans = apply(&OPS, &MATRIX);
        assert_eq!(ans, [33210, 490, 4243455, 401]);
    }

    #[test]
    fn test_parse() {
        let (_, (ms, os)) = parse_problems(TEXT).unwrap();
        assert_eq!(ms, MATRIX);
        assert_eq!(os, OPS);
    }
}
