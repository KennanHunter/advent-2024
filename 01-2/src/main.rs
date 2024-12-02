#![feature(iter_next_chunk, binary_heap_into_iter_sorted)]

use std::collections::{BinaryHeap, HashMap};
use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();

    let mut left = BinaryHeap::<u64>::new();
    let mut right = HashMap::<u64, u64>::new();

    for [left_item, right_item] in input
        .lines()
        .map(|line| line.split("   ").next_chunk::<2>().unwrap())
    {
        left.push(left_item.parse().unwrap());

        right
            .entry(right_item.parse().unwrap())
            .and_modify(|val| *val += 1)
            .or_insert(1);
    }

    let sum: u64 = left
        .into_iter()
        .map(|val| val * right.get(&val).unwrap_or(&0))
        .sum();

    println!("The total similarity score between the two lists is {sum}");
}
