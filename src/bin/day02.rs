use std::{collections::HashSet, ops::RangeInclusive};

use rust_aoc::read_csv_into_lines;

const DAY: u8 = 2;

fn main() {
    let ex_lines = read_csv_into_lines(DAY, true);
    println!("Part 1 result: {}", solve_part_1(&ex_lines));
    println!("Part 2 result: {}", solve_part_2(&ex_lines));

    let lines = read_csv_into_lines(DAY, false);
    println!("Part 1 result: {}", solve_part_1(&lines));
    println!("Part 2 result: {}", solve_part_2(&lines));
}

fn parse_input(input: &str) -> RangeInclusive<i64> {
    let parts = input.split("-");
    let from_to: Vec<i64> = parts
        .map(|num_str| num_str.parse::<i64>().unwrap())
        .collect();
    return from_to[0]..=from_to[1];
}

fn check_if_num_invalid_p1(num: i64) -> bool {
    let num_str = num.to_string();
    let str_len = num_str.len();
    if str_len % 2 != 0 {
        return false;
    }
    let (first, last) = num_str.split_at(str_len / 2);
    if first == last {
        return true;
    }
    return false;
}

fn create_chunk_set(str: &String, chunk_size: usize) -> HashSet<String> {
    assert!(chunk_size > 0);
    let mut chunks = HashSet::new();
    let mut current_chunk = String::new();

    for c in str.chars() {
        current_chunk.push(c);
        if current_chunk.len() == chunk_size {
            chunks.insert(current_chunk);
            current_chunk = String::new();
        }
    }

    return chunks;
}

fn check_if_num_invalid_p2(num: i64) -> bool {
    let num_str = num.to_string();
    let str_len = num_str.len();
    for pattern_len in 1..=(str_len / 2) {
        if str_len % pattern_len != 0 {
            continue;
        }

        if create_chunk_set(&num_str, pattern_len).len() == 1 {
            return true;
        }
    }
    return false;
}

fn solve_part_1(lines: &Vec<String>) -> i64 {
    let mut invalid_ids: Vec<i64> = Vec::new();
    for input in lines {
        for id in parse_input(input) {
            if check_if_num_invalid_p1(id) {
                invalid_ids.push(id);
            }
        }
    }
    return invalid_ids.iter().sum();
}

fn solve_part_2(lines: &Vec<String>) -> i64 {
    let mut invalid_ids: Vec<i64> = Vec::new();
    for input in lines {
        for id in parse_input(input) {
            if check_if_num_invalid_p2(id) {
                invalid_ids.push(id);
            }
        }
    }
    return invalid_ids.iter().sum();
}
