// Advent of Code 2025 Day 7
// A. Drew

fn main() {
    let text = std::fs::read_to_string("data/day07/input.txt").unwrap();
    let (initial_beam, splitters) = parse_input(&text).unwrap();
    let ans0 = count_splits(&initial_beam, &splitters);
    let ans1 = count_timelines(&initial_beam, &splitters);
    println!("{}\n{}", ans0, ans1);
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

fn count_timelines<T>(initial_beam: &isize, splitters: &[T]) -> usize
where
    T: AsRef<[isize]>,
{
    let beams = splitters
        .iter()
        .fold(vec![(*initial_beam, 1)], |beams, splitters| {
            let (_, beams) = split_beams(&beams, splitters.as_ref());
            beams
        });
    beams.iter().map(|(_, cnt)| cnt).sum()
}

fn count_splits<T>(initial_beam: &isize, splitters: &[T]) -> usize
where
    T: AsRef<[isize]>,
{
    let (sum, _) =
        splitters
            .iter()
            .fold((0, vec![(*initial_beam, 1)]), |(sum, beams), splitters| {
                let (n, beams) = split_beams(&beams, splitters.as_ref());
                (sum + n, beams)
            });
    sum
}

fn split_beams(beams: &[(isize, usize)], splitters: &[isize]) -> (usize, Vec<(isize, usize)>) {
    let mut splits = Vec::new();
    let mut beams: Vec<_> = beams
        .iter()
        .flat_map(|b @ (pos, _)| {
            if splitters.binary_search(pos).is_ok() {
                splits.push(pos);
                split_beam(b).to_vec()
            } else {
                vec![*b]
            }
        })
        .collect();
    splits.sort();
    splits.dedup();
    beams.sort();
    beams = beams
        .chunk_by(|a, b| a == b)
        .map(|c| {
            (
                c.first().expect("chunk has at least one element").0,
                c.iter().map(|(_, n)| n).sum(),
            )
        })
        .collect();
    (splits.len(), beams)
}

fn split_beam((pos, cnt): &(isize, usize)) -> [(isize, usize); 2] {
    [(pos - 1, *cnt), (pos + 1, *cnt)]
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
    fn test_count_timelines() {
        assert_eq!(count_timelines(&INITIAL_BEAM, &SPLITTERS), 40);
    }

    #[test]
    fn test_parse() {
        let (initial_beam, splitters) = parse_input(TEXT).unwrap();
        assert_eq!(initial_beam, INITIAL_BEAM);
        assert_eq!(splitters, SPLITTERS);
    }
}
