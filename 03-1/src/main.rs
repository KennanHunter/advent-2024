#![feature(iter_next_chunk, binary_heap_into_iter_sorted)]
use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();

    let mut search_iter = input.chars();

    let mut sum = 0;

    'main: loop {
        if search_iter.clone().next_chunk::<4>().unwrap_or_default() == ['m', 'u', 'l', '('] {
            let _ = search_iter.next_chunk::<4>();
        } else {
            if let None = search_iter.next() {
                break 'main;
            } else {
                continue 'main;
            }
        }

        let mut left_num_string = String::new();
        let mut right_num_string = String::new();

        'num: loop {
            match search_iter.next() {
                Some(ch) if ch.is_ascii_alphanumeric() => left_num_string.push(ch),
                Some(',') => break 'num,
                Some(_) => {
                    search_iter.next();

                    continue 'main;
                }
                None => break 'main,
            };
        }

        'num: loop {
            match search_iter.next() {
                Some(ch) if ch.is_ascii_alphanumeric() => right_num_string.push(ch),
                Some(')') => break 'num,
                Some(_) => {
                    search_iter.next();

                    continue 'main;
                }
                None => break 'main,
            };
        }

        let left = left_num_string.parse::<usize>().unwrap();
        let right = right_num_string.parse::<usize>().unwrap();

        println!("left: {left}, right: {right}");

        sum += left * right;
    }

    println!("Sum: {sum}")
}
