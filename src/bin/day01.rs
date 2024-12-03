use itertools::Itertools;
use std::iter::zip;

fn main() {
    let input = std::fs::read_to_string("inputs/day01/input.txt").unwrap();

    let (left, right): (Vec<_>, Vec<_>) = input
        .lines()
        .filter_map(|line| line.split_once(char::is_whitespace))
        .unzip();

    let left: Vec<i32> = left
        .into_iter()
        .filter_map(|n| n.parse().ok())
        .sorted()
        .collect();
    let right: Vec<i32> = right
        .into_iter()
        .filter_map(|n| n.trim().parse().ok())
        .sorted()
        .collect();

    let solution_a: u32 = zip(&left, &right).map(|(l, r)| l.abs_diff(*r)).sum();
    println!("Solution 1: {}", solution_a);

    let mut solution_b = 0;
    for left in left {
        for right in &right {
            if left == *right {
                solution_b += right;
            }
        }
    }
    println!("Solution 2: {}", solution_b);
}
