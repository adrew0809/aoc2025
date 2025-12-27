// Advent of Code 2025 Day 7
// A. Drew

fn main() {
    let text = std::fs::read_to_string("data/day07/input.txt").unwrap();
    let (initial_beam, splitters) = parse_input(&text).unwrap();
    let ans0 = count_splits(&initial_beam, &splitters);
    println!("{}", ans0);
}

fn parse_input(text: &str) -> Option<(isize, Vec<Vec<isize>>)> {
    let lines: Vec<_> = text.lines().collect();
    lines.split_first().and_then(|(first, rest)| {
        find_initial_beam(first).map(|initial_beam| (initial_beam, parse_splitters(rest)))
    })
}

fn find_initial_beam(line: &str) -> Option<isize> {
    line.chars()
        .position(|c| c == 'S')
        .and_then(|i| isize::try_from(i).ok())
}

fn parse_splitters(lines: &[&str]) -> Vec<Vec<isize>> {
    lines
        .iter()
        .map(|line| {
            line.char_indices()
                .filter_map(|(i, c)| {
                    if c == '^' {
                        isize::try_from(i).ok()
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn count_splits<T>(initial_beam: &isize, splitters: &[T]) -> usize
where
    T: AsRef<[isize]>,
{
    let (sum, _) = splitters
        .iter()
        .fold((0, vec![*initial_beam]), |(sum, beams), splitters| {
            let (n, beams) = split_beams(&beams, splitters.as_ref());
            (sum + n, beams)
        });
    sum
}

fn split_beams(beams: &[isize], splitters: &[isize]) -> (usize, Vec<isize>) {
    let mut num_splits = 0;
    let mut beams: Vec<_> = beams
        .iter()
        .flat_map(|b| {
            if splitters.binary_search(b).is_ok() {
                num_splits += 1;
                split_beam(b).to_vec()
            } else {
                vec![*b]
            }
        })
        .collect();
    beams.sort();
    beams.dedup();
    (num_splits, beams)
}

fn split_beam(beam: &isize) -> [isize; 2] {
    [beam - 1, beam + 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXT: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    const INITIAL_BEAM: isize = 7;

    const SPLITTERS: [&[isize]; 15] = [
        &[],
        &[7],
        &[],
        &[6, 8],
        &[],
        &[5, 7, 9],
        &[],
        &[4, 6, 10],
        &[],
        &[3, 5, 9, 11],
        &[],
        &[2, 6, 12],
        &[],
        &[1, 3, 5, 7, 9, 13],
        &[],
    ];

    #[test]
    fn test_count_splits() {
        assert_eq!(count_splits(&INITIAL_BEAM, &SPLITTERS), 21);
    }

    #[test]
    fn test_parse() {
        let (initial_beam, splitters) = parse_input(TEXT).unwrap();
        assert_eq!(initial_beam, INITIAL_BEAM);
        assert_eq!(splitters, SPLITTERS);
    }
}
