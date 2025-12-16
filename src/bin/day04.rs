// Advent of Code 2025 Day 4
// A. Drew

fn main() {
    let accessible = std::fs::read_to_string("data/day04/input.txt").map(|text| {
        let rolls = parse_rolls(&text);
        find_accessible(&rolls).len()
    });
    println!("{:?}", accessible);
}

fn parse_rolls(text: &str) -> Vec<[isize; 2]> {
    text.lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().filter_map(move |(j, c)| match c {
                '@' => Some([i as isize, j as isize]),
                _ => None,
            })
        })
        .collect()
}

fn find_accessible(rolls: &[[isize; 2]]) -> Vec<[isize; 2]> {
    rolls
        .iter()
        .filter(|[i, j]| {
            let neighbors = (i - 1..=i + 1)
                .flat_map(|ii| (j - 1..=j + 1).map(move |jj| [ii, jj]))
                .filter(|x| rolls.binary_search(x).is_ok())
                .count();
            neighbors < (4 + 1)
        })
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    const ROLLS: [[isize; 2]; 71] = [
        [0, 2],
        [0, 3],
        [0, 5],
        [0, 6],
        [0, 7],
        [0, 8],
        [1, 0],
        [1, 1],
        [1, 2],
        [1, 4],
        [1, 6],
        [1, 8],
        [1, 9],
        [2, 0],
        [2, 1],
        [2, 2],
        [2, 3],
        [2, 4],
        [2, 6],
        [2, 8],
        [2, 9],
        [3, 0],
        [3, 2],
        [3, 3],
        [3, 4],
        [3, 5],
        [3, 8],
        [4, 0],
        [4, 1],
        [4, 3],
        [4, 4],
        [4, 5],
        [4, 6],
        [4, 8],
        [4, 9],
        [5, 1],
        [5, 2],
        [5, 3],
        [5, 4],
        [5, 5],
        [5, 6],
        [5, 7],
        [5, 9],
        [6, 1],
        [6, 3],
        [6, 5],
        [6, 7],
        [6, 8],
        [6, 9],
        [7, 0],
        [7, 2],
        [7, 3],
        [7, 4],
        [7, 6],
        [7, 7],
        [7, 8],
        [7, 9],
        [8, 1],
        [8, 2],
        [8, 3],
        [8, 4],
        [8, 5],
        [8, 6],
        [8, 7],
        [8, 8],
        [9, 0],
        [9, 2],
        [9, 4],
        [9, 5],
        [9, 6],
        [9, 8],
    ];

    #[test]
    fn test_parsing() {
        let rolls = parse_rolls(CONTENT);
        assert_eq!(rolls, ROLLS);
    }

    #[test]
    fn test_sample_0() {
        let [i, j] = [1, 2];
        let neighbors = (i - 1..=i + 1)
            .flat_map(|ii| (j - 1..=j + 1).map(move |jj| [ii, jj]))
            .collect::<Vec<_>>();
        assert_eq!(
            neighbors,
            [
                [0, 1],
                [0, 2],
                [0, 3],
                [1, 1],
                [1, 2],
                [1, 3],
                [2, 1],
                [2, 2],
                [2, 3]
            ]
        );
    }

    #[test]
    fn test_sample() {
        let accessible = find_accessible(&ROLLS);
        assert_eq!(
            accessible,
            [
                [0, 2],
                [0, 3],
                [0, 5],
                [0, 6],
                [0, 8],
                [1, 0],
                [2, 6],
                [4, 0],
                [4, 9],
                [7, 0],
                [9, 0],
                [9, 2],
                [9, 8]
            ]
        );
    }
}
