// Advent of Code 2025 Day 3
// A. Drew

fn main() {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let joltage: u32 =
        BufReader::new(File::open("data/day03/input.txt").expect("can read input file"))
            .lines()
            .map(|line| {
                largest_joltage(
                    &line
                        .expect("can read input file")
                        .chars()
                        .map(|c| c.to_digit(10).expect("input is all digits"))
                        .collect::<Vec<_>>(),
                )
                .expect("input has no empty lines")
            })
            .sum();
    println!("joltage: {}", joltage);
}

fn largest_joltage(bank: &[u32]) -> Option<u32> {
    let n = bank.len();
    bank.iter()
        .bounded_max_element(0, n - 1, strict_gt)
        .and_then(|(i, a)| {
            bank.iter()
                .bounded_max_element(i + 1, n, strict_gt)
                .map(|(_, b)| 10 * a + b)
        })
    //.and_then(|[a, b]| {
    //    a.to_digit(10)
    //        .and_then(|a| b.to_digit(10).map(|b| 10 * a + b))
    //})
}

fn strict_gt<T: PartialOrd>(x: &T, y: &T) -> std::cmp::Ordering {
    if x < y {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Greater
    }
}

trait BoundedMaxIter {
    fn bounded_max_element<F>(
        self,
        start: usize,
        end: usize,
        compare: F,
    ) -> Option<(usize, Self::Item)>
    where
        Self: Sized + Iterator,
        F: Fn(&Self::Item, &Self::Item) -> std::cmp::Ordering,
    {
        self.enumerate()
            .skip(start)
            .take(end - start)
            .max_by(|(_, x), (_, y)| compare(x, y))
    }
}

impl<I> BoundedMaxIter for I where I: Iterator {}

#[cfg(test)]
mod tests {
    use super::*;

    const BANKS: [[u32; 15]; 4] = [
        [9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
        [8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
        [2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
        [8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
    ];

    #[test]
    fn test_sample() {
        let js: Vec<_> = BANKS.iter().map(|bank| largest_joltage(bank)).collect();
        assert_eq!(js, [Some(98), Some(89), Some(78), Some(92)]);
    }
}
