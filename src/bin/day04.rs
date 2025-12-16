// Advent of Code 2025 Day 4
// A. Drew

fn main() {
    use std::ops::ControlFlow;

    let answer0 = std::fs::read_to_string("data/day04/input.txt").map(|text| {
        let rolls = parse_rolls(&text);
        let (accessible, _) = partition_accessible(&rolls);
        accessible.len()
    });
    println!("{:?}", answer0);

    let answer1 = std::fs::read_to_string("data/day04/input.txt").map(|text| {
        let rolls = parse_rolls(&text);
        std::iter::repeat(()).try_fold((0, rolls), |(cnt, rolls), _| {
            let (removed, remaining) = partition_accessible(&rolls);
            let n = removed.len();
            if n == 0 {
                ControlFlow::Break(cnt)
            } else {
                ControlFlow::Continue((cnt + n, remaining))
            }
        })
    });

    println!("{:?}", answer1);
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

// return the accessible rolls that were removed and the remaining rolls
fn partition_accessible(rolls: &[[isize; 2]]) -> (Vec<[isize; 2]>, Vec<[isize; 2]>) {
    rolls.iter().partition(|[i, j]| {
        let neighbors = (i - 1..=i + 1)
            .flat_map(|ii| (j - 1..=j + 1).map(move |jj| [ii, jj]))
            .filter(|x| rolls.binary_search(x).is_ok())
            .count();
        neighbors < (4 + 1)
    })
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
    fn test_sample() {
        let (accessible, _) = partition_accessible(&ROLLS);
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

    #[test]
    fn test_sample_1() {
        let (accessible, left) = partition_accessible(&ROLLS);
        assert_eq!(accessible.len(), 13);
        let (accessible, left) = partition_accessible(&left);
        assert_eq!(accessible.len(), 12);
        let (accessible, left) = partition_accessible(&left);
        assert_eq!(accessible.len(), 7);
        let (accessible, left) = partition_accessible(&left);
        assert_eq!(accessible.len(), 5);
        let (accessible, left) = partition_accessible(&left);
        assert_eq!(accessible.len(), 2);
        let (accessible, left) = partition_accessible(&left);
        assert_eq!(accessible.len(), 1);
        let (accessible, left) = partition_accessible(&left);
        assert_eq!(accessible.len(), 1);
        let (accessible, left) = partition_accessible(&left);
        assert_eq!(accessible.len(), 1);
        let (accessible, left) = partition_accessible(&left);
        assert_eq!(accessible.len(), 1);
        let (accessible, left) = partition_accessible(&left);
        assert_eq!(accessible.len(), 0);
        assert_eq!(
            left,
            [
                [3, 4],
                [3, 5],
                [4, 3],
                [4, 4],
                [4, 5],
                [4, 6],
                [5, 3],
                [5, 4],
                [5, 5],
                [5, 6],
                [5, 7],
                [6, 3],
                [6, 5],
                [6, 7],
                [6, 8],
                [7, 3],
                [7, 4],
                [7, 6],
                [7, 7],
                [7, 8],
                [8, 3],
                [8, 4],
                [8, 5],
                [8, 6],
                [8, 7],
                [9, 4],
                [9, 5],
                [9, 6],
            ]
        );
    }
}
