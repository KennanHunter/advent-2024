#![feature(iter_map_windows)]

use std::cmp::Ordering;
use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();

    let mut safe_lines = 0;

    for line in input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|elements| elements.map(|item| item.parse::<i64>().unwrap()))
    {
        let all_increasing = line.clone().map_windows(|[first, second]| {
            return second > first;
        });
    }

    println!("The total amount of safe lines is {}", safe_lines);
}
