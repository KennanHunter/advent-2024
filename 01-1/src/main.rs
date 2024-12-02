#![feature(iter_next_chunk, binary_heap_into_iter_sorted)]

use std::collections::BinaryHeap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();

    let [left, right] = input
        .lines()
        .map(|line| line.split("   ").next_chunk::<2>().unwrap())
        .fold(
            [BinaryHeap::<u64>::new(), BinaryHeap::<u64>::new()],
            |[mut left_heap, mut right_heap], [left_item, right_item]| {
                left_heap.push(left_item.parse().unwrap());
                right_heap.push(right_item.parse().unwrap());

                [left_heap, right_heap]
            },
        );

    let sum: u64 = Iterator::zip(left.into_iter_sorted(), right.into_iter_sorted())
        .map(|(left, right)| u64::abs_diff(left, right))
        .sum();

    println!("The total distance between the two lists is {sum}");
}
