// Advent of Code 2025 Day 6
// A. Drew

use std::ops::{Add, Mul};

type Op = fn(u64, u64) -> u64;

fn main() {
    let text = std::fs::read_to_string("data/day06/input.txt").unwrap();
    let lines: Vec<_> = text.lines().collect();
    let (operator_line, operand_lines) = lines.split_last().unwrap();
    let operators = parse_operators(operator_line);
    let ans0: u64 = solve_rows(&operators, operand_lines).iter().sum();
    let ans1: u64 = solve_cols(&operators, operand_lines).iter().sum();
    println!("answer 0: {}", ans0);
    println!("answer 1: {}", ans1);
}

fn solve_rows(operators: &[(usize, usize, Op)], operand_lines: &[&str]) -> Vec<u64> {
    operators
        .iter()
        .map(|(i, w, op)| {
            operand_lines
                .iter()
                .map(move |line| line.chars().skip(*i).take(*w).collect::<String>())
                .map(|s| s.trim().parse().expect("all digit input"))
                .reduce(op)
                .expect("at least one line")
        })
        .collect()
}

fn solve_cols(operators: &[(usize, usize, Op)], operand_lines: &[&str]) -> Vec<u64> {
    operators
        .iter()
        .map(|(i, w, op)| {
            (0..*w)
                .map(|j| {
                    let digits: String = operand_lines
                        .iter()
                        .map(|line| line.chars().nth(i + j).expect("digits all in line"))
                        .collect();
                    digits.trim().parse().expect("all digit input")
                })
                .reduce(op)
                .expect("at least one line")
        })
        .collect()
}

fn parse_operators(input: &str) -> Vec<(usize, usize, Op)> {
    input
        .char_indices()
        .fold(Vec::<(usize, usize, Op)>::new(), |mut acc, (i, c)| {
            match c {
                '*' => {
                    if let Some((_, w, _)) = acc.last_mut() {
                        *w -= 1;
                    }
                    acc.push((i, 1, u64::mul));
                }
                '+' => {
                    if let Some((_, w, _)) = acc.last_mut() {
                        *w -= 1;
                    }
                    acc.push((i, 1, u64::add));
                }
                _ => {
                    if let Some((_, w, _)) = acc.last_mut() {
                        *w += 1;
                    }
                }
            }
            acc
        })
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

    #[test]
    fn test_paser_operators() {
        let line = "*   +    *   +  ";
        let ops = parse_operators(line);
        assert_eq!(ops.len(), 4);
        let os: Vec<_> = ops.iter().map(|(i, w, _)| (*i, *w)).collect();
        assert_eq!(os, [(0, 3), (4, 4), (9, 3), (13, 3)]);
    }

    #[test]
    fn test_solve_rows() {
        let lines: Vec<_> = TEXT.lines().collect();
        let (operator_line, operand_lines) = lines.split_last().unwrap();
        let operators = parse_operators(operator_line);
        let ans = solve_rows(&operators, operand_lines);
        assert_eq!(ans, [33210, 490, 4243455, 401]);
    }

    #[test]
    fn test_solve_cols() {
        let lines: Vec<_> = TEXT.lines().collect();
        let (operator_line, operand_lines) = lines.split_last().unwrap();
        let operators = parse_operators(operator_line);
        let ans = solve_cols(&operators, operand_lines);
        assert_eq!(ans, [8544, 625, 3253600, 1058]);
    }
}
