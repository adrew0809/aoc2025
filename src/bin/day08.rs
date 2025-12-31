// Advent of Code 2025 Day 8
// A. Drew

use std::collections::{BTreeSet, VecDeque};
use std::iter::zip;

type Position = [i64; 3];
type Weight = i64;
type Edge = (Weight, [Position; 2]);

fn main() {
    let text = std::fs::read_to_string("data/day08/input.txt").unwrap();
    let boxes: Vec<_> = text
        .lines()
        .map(|line| {
            let nums: Vec<_> = line.split(',').map(|d| d.parse::<i64>().unwrap()).collect();
            assert_eq!(nums.len(), 3);
            [nums[0], nums[1], nums[2]]
        })
        .collect();
    let edges = get_shortest(1000, &boxes);
    let circuits = connect(&edges);
    let mut sizes: Vec<_> = circuits.iter().map(|x| x.len()).collect();
    sizes.sort_by_key(|a| std::cmp::Reverse(*a));
    let ans0: usize = sizes.iter().take(3).product();
    println!("answer 0: {}", ans0);
}

fn get_shortest(n: usize, boxes: &[Position]) -> Vec<Edge> {
    boxes
        .iter()
        .enumerate()
        .fold(VecDeque::<Edge>::with_capacity(n), |mut edges, (i, a)| {
            boxes.iter().skip(i + 1).for_each(|b| {
                let weight: i64 = zip(a, b).map(|(a, b)| (a - b).pow(2)).sum();
                let mut vertices = [*a, *b];
                vertices.sort();
                let j = edges.partition_point(|(w, _)| *w <= weight);
                edges.insert(j, (weight, vertices));
                edges.truncate(n);
            });
            edges
        })
        .into()
}

fn connect(edges: &[Edge]) -> Vec<BTreeSet<Position>> {
    edges
        .iter()
        .fold(Vec::<BTreeSet<Position>>::new(), |sets, (_, [a, b])| {
            let (mut with, mut without): (Vec<_>, Vec<_>) = sets
                .iter()
                .cloned()
                .partition(|x| x.contains(a) || x.contains(b));
            with.push(BTreeSet::from([*a, *b]));
            let merged = with
                .iter()
                .fold(BTreeSet::new(), |acc, x| acc.union(x).cloned().collect());
            without.push(merged);
            without
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const BOXES: [Position; 20] = [
        [162, 817, 812],
        [57, 618, 57],
        [906, 360, 560],
        [592, 479, 940],
        [352, 342, 300],
        [466, 668, 158],
        [542, 29, 236],
        [431, 825, 988],
        [739, 650, 466],
        [52, 470, 668],
        [216, 146, 977],
        [819, 987, 18],
        [117, 168, 530],
        [805, 96, 715],
        [346, 949, 466],
        [970, 615, 88],
        [941, 993, 340],
        [862, 61, 35],
        [984, 92, 344],
        [425, 690, 689],
    ];

    #[test]
    fn test_get_shortest() {
        let edges = get_shortest(10, &BOXES);
        let edges: Vec<_> = edges.iter().map(|(_, pos)| *pos).collect();
        assert_eq!(
            edges[..4],
            [
                [[162, 817, 812], [425, 690, 689]],
                [[162, 817, 812], [431, 825, 988]],
                [[805, 96, 715], [906, 360, 560]],
                [[425, 690, 689], [431, 825, 988]]
            ]
        );
    }

    #[test]
    fn test_connect() {
        let edges = get_shortest(10, &BOXES);
        let circuits = connect(&edges);
        let mut sizes: Vec<_> = circuits.iter().map(|x| x.len()).collect();
        sizes.sort_by_key(|a| std::cmp::Reverse(*a));
        assert_eq!(sizes, [5, 4, 2, 2]);
    }
}
