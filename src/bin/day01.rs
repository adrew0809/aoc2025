// Advent of Code 2025 Day 1
// A. Drew

fn main() {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let rotations = BufReader::new(File::open("data/day01/input.txt").unwrap())
        .lines()
        .map(|line| parse_rotation(&line.unwrap()).unwrap());
    let passwords = process(rotations).fold([0, 0], |passwords, (position, clicks)| {
        [
            passwords[0] + if position == 0 { 1 } else { 0 },
            passwords[1] + clicks,
        ]
    });
    println!("passwords: {:?}", passwords);
}

fn parse_rotation(line: &str) -> Result<isize, nom::Err<nom::error::Error<&str>>> {
    use nom::{
        character::complete::{char, digit1},
        combinator::all_consuming,
        Parser,
    };
    let direction = char('L').map(|_| -1).or(char('R').map(|_| 1));
    let magnitude = digit1.map_res(str::parse::<isize>);
    let rotation = direction.and(magnitude).map(|(d, m)| d * m);
    all_consuming(rotation).parse(line).map(|(_, x)| x)
}

// Used to iterate over the positions and clicks from initial position 50 using the input rotations
fn process(rotations: impl IntoIterator<Item = isize>) -> impl Iterator<Item = (isize, isize)> {
    rotations.into_iter().scan(50, |position_prev, rotation| {
        let clicks = count_clicks(*position_prev, rotation);
        let position = (*position_prev + rotation).rem_euclid(N);
        *position_prev = position;
        Some((position, clicks))
    })
}

fn count_clicks(position: isize, rotation: isize) -> isize {
    if rotation.is_negative() {
        // translate to an equivalent positive rotation
        ((-position).rem_euclid(N) - rotation) / N
    } else {
        (position + rotation) / N
    }
}

const N: isize = 100;

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    const ROTATIONS: [isize; 10] = [-68, -30, 48, -5, 60, -55, -1, -99, 14, -82];

    #[test]
    fn test_parse_sample() {
        let rotations: Vec<_> = CONTENT
            .lines()
            .map(|line| parse_rotation(line).unwrap())
            .collect();
        assert_eq!(rotations, ROTATIONS);
    }

    #[test]
    fn test_process_sample() {
        let xs: Vec<_> = process(ROTATIONS).collect();
        assert_eq!(
            xs,
            [
                (82, 1),
                (52, 0),
                (0, 1),
                (95, 0),
                (55, 1),
                (0, 1),
                (99, 0),
                (0, 1),
                (14, 0),
                (32, 1)
            ]
        );
    }

    #[test]
    fn test_clicks_with_cycles() {
        assert_eq!(count_clicks(0, 300), 3);
        assert_eq!(count_clicks(10, -110), 2);
    }
}
