// Advent of Code 2025 Day 8
// A. Drew

use std::cmp::{Ordering, Reverse};
use std::collections::{BTreeSet, BinaryHeap};
use std::iter::zip;

type Node = [i64; 3];
type Weight = i64;

#[derive(Clone, Copy)]
struct Edge(Weight, [Node; 2]);

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Edge {}

fn main() {
    let text = std::fs::read_to_string("data/day08/input.txt").unwrap();
    let boxes = parse_boxes(&text).unwrap();
    let mut edges = get_edges(&boxes);
    let mut circuits = connect_n(&mut edges.clone(), 1000);
    circuits.sort_by_key(|b| std::cmp::Reverse(b.len()));
    let ans0: usize = circuits.iter().take(3).map(|a| a.len()).product();
    println!("answer 0: {}", ans0);
    let whole_circuit = connect_all(&mut edges, boxes.len());
    let [[x0, _, _], [x1, _, _]] = whole_circuit.last().unwrap();
    let ans1 = x0 * x1;
    println!("answer 1: {}", ans1);
}

// Connect the n shortest nodes
fn connect_n(edges: &mut BinaryHeap<Reverse<Edge>>, n: usize) -> Vec<BTreeSet<Node>> {
    let mut circuits: Vec<BTreeSet<Node>> = Vec::new();
    for _ in 0..n {
        if let Some(Reverse(Edge(_, nodes))) = edges.pop() {
            circuits = add_edge(&circuits, &nodes);
        }
    }
    circuits
}

fn add_edge(circuits: &[BTreeSet<Node>], [a, b]: &[Node; 2]) -> Vec<BTreeSet<Node>> {
    let (mut with, mut without): (Vec<_>, Vec<_>) = circuits
        .iter()
        .cloned()
        .partition(|x| x.contains(a) || x.contains(b));
    with.push(BTreeSet::from([*a, *b]));
    let merged = with
        .iter()
        .fold(BTreeSet::new(), |acc, x| acc.union(x).cloned().collect());
    without.push(merged);
    without
}

// Get a min-heap of edges from a slice of nodes
fn get_edges(boxes: &[Node]) -> BinaryHeap<Reverse<Edge>> {
    boxes
        .iter()
        .enumerate()
        .fold(BinaryHeap::new(), |edges, (i, a)| {
            boxes.iter().skip(i + 1).fold(edges, |mut edges, b| {
                let weight: i64 = zip(a, b).map(|(a, b)| (a - b).pow(2)).sum();
                let mut vertices = [*a, *b];
                vertices.sort();
                edges.push(Reverse(Edge(weight, vertices)));
                edges
            })
        })
}

// Connect the nodes until there is one circuit of size n
fn connect_all(edges: &mut BinaryHeap<Reverse<Edge>>, n: usize) -> Vec<[Node; 2]> {
    let mut connected_edges: Vec<[Node; 2]> = Vec::new();
    let mut circuits: Vec<BTreeSet<Node>> = Vec::new();
    while !(circuits.len() == 1 && circuits.first().unwrap().len() == n) {
        if let Some(Reverse(Edge(_, nodes))) = edges.pop() {
            connected_edges.push(nodes);
            circuits = add_edge(&circuits, &nodes);
        }
    }
    connected_edges
}

fn parse_boxes(text: &str) -> Option<Vec<Node>> {
    text.lines()
        .map(|line| {
            let nums: Option<Vec<_>> = line.split(',').map(|d| d.parse::<i64>().ok()).collect();
            nums.filter(|x| x.len() == 3).map(|x| [x[0], x[1], x[2]])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const BOXES: [Node; 20] = [
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
    fn test_connect_n() {
        let mut edges = get_edges(&BOXES);
        assert_eq!(edges.len(), (20 * 19) / 2);
        let circuits = connect_n(&mut edges, 10);
        let mut sizes: Vec<_> = circuits.iter().map(|x| x.len()).collect();
        sizes.sort_by_key(|a| std::cmp::Reverse(*a));
        assert_eq!(sizes, [5, 4, 2, 2]);
    }

    #[test]
    fn test_get_edges() {
        let edges = get_edges(&BOXES);
        let Reverse(Edge(_, nodes)) = edges.peek().unwrap();
        assert_eq!(nodes, &[[162, 817, 812], [425, 690, 689]]);
    }

    #[test]
    fn test_connect_all() {
        let mut edges = get_edges(&BOXES);
        let circuit = connect_all(&mut edges, BOXES.len());
        assert_eq!(circuit.last().unwrap(), &[[117, 168, 530], [216, 146, 977]]);
    }
}
