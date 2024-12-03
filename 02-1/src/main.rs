#![feature(iter_map_windows)]

use std::cmp::Ordering;
use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();

    let mut safe_lines = 0;

    for line in input.lines().map(|line| line.split_whitespace()) {
        let mut direction: Option<Ordering> = None;

        let safe = line
            .map(|item| item.parse::<i64>().unwrap())
            .map_windows(|[first, second]| {
                if !matches!(first.abs_diff(*second), 1..=3) {
                    return false;
                }

                if let Some(past_direction) = direction {
                    past_direction == first.cmp(second)
                } else {
                    let _ = direction.insert(first.cmp(second));
                    true
                }
            })
            .all(|f| f);

        if safe {
            safe_lines += 1;
        }
    }

    println!("The total amount of safe lines is {}", safe_lines);
}
