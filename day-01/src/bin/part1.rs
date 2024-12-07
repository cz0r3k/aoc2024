#![feature(test)]
use std::collections::BinaryHeap;

extern crate test;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    println!("{output}");
}

fn part1_heap(input: &str) -> String {
    let (left_numbers, right_numbers) = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(left, right)| (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap()))
        .unzip::<i32, i32, BinaryHeap<i32>, BinaryHeap<i32>>();
    left_numbers
        .iter()
        .zip(right_numbers.iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>()
        .to_string()
}

fn part1(input: &str) -> String {
    let (mut left, mut right) = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(left, right)| (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap()))
        .unzip::<i32, i32, Vec<i32>, Vec<i32>>();
    left.sort_unstable();
    right.sort_unstable();
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part1_heap};
    use test::Bencher;

    #[test]
    fn part1_test() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("11", part1(input));
    }
    #[bench]
    fn part1_bench(b: &mut Bencher) {
        let input = include_str!("./input.txt");
        b.iter(|| part1(input));
    }

    #[bench]
    fn part1_heap_bench(b: &mut Bencher) {
        let input = include_str!("./input.txt");
        b.iter(|| part1_heap(input));
    }
}
