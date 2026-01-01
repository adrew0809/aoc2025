// Advent of Code 2025 Day 9
// A. Drew

fn main() -> Result<(), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let reader = BufReader::new(File::open("data/day09/input.txt")?);
    let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
    let tiles = lines
        .iter()
        .map(|line| parse_tile(&line))
        .collect::<Result<Vec<_>, _>>()?;
    let ans0 = largest_rectangle(&tiles).as_ref().map(|x| area(x));
    println!("answer 0: {:?}", ans0);
    Ok(())
}

fn largest_rectangle(tiles: &[[i64; 2]]) -> Option<[[i64; 2]; 2]> {
    tiles
        .iter()
        .enumerate()
        .flat_map(|(i, a)| tiles.iter().skip(i + 1).map(move |b| [a, b]))
        .max_by_key(|[a, b]| area(&[**a, **b]))
        .map(|[a, b]| [*a, *b])
}

fn area([[x0, y0], [x1, y1]]: &[[i64; 2]; 2]) -> i64 {
    ((x1 - x0).abs() + 1) * ((y1 - y0).abs() + 1)
}

fn parse_tile(line: &str) -> Result<[i64; 2], Error> {
    let elements = line
        .split(',')
        .map(parse_coord)
        .collect::<Result<Vec<_>, _>>()?;
    let pair = elements.try_into()?;
    Ok(pair)
}

fn parse_coord(input: &str) -> Result<i64, Error> {
    let n = input.parse()?;
    Ok(n)
}

#[derive(Debug)]
enum Error {
    Number(std::num::ParseIntError),
    Elements(Vec<i64>),
    Read(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Number(e) => e.fmt(f),
            Error::Elements(e) => write!(f, "expected 2 elements, found {}", e.len()),
            Error::Read(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Error::Number(value)
    }
}

impl From<Vec<i64>> for Error {
    fn from(value: Vec<i64>) -> Self {
        Error::Elements(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Read(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TILES: [[i64; 2]; 8] = [
        [7, 1],
        [11, 1],
        [11, 7],
        [9, 7],
        [9, 5],
        [2, 5],
        [2, 3],
        [7, 3],
    ];

    #[test]
    fn test_sample() {
        let pair = largest_rectangle(&TILES).unwrap();
        assert_eq!(area(&pair), 50);
    }

    #[test]
    fn test_parse() {
        let pair = parse_tile("7,1").unwrap();
        assert_eq!(pair, [7, 1]);
    }
}
