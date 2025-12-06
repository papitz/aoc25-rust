use std::{cmp::max, ops::RangeInclusive};

use rust_aoc::{parse_iclusive_range, read_lines};

const DAY: u8 = 5;

fn main() {
    let ex_lines = read_lines(DAY, true);
    println!("Part 1 result: {}", solve_part_1(&ex_lines));
    println!("Part 2 result: {}", solve_part_2(&ex_lines));

    let lines = read_lines(DAY, false);
    println!("Part 1 result: {}", solve_part_1(&lines));
    println!("Part 2 result: {}", solve_part_2(&lines));
}

fn get_ingredients_and_fresh_map(lines: &Vec<String>) -> (Vec<i64>, Vec<RangeInclusive<i64>>) {
    let mut ingredients: Vec<i64> = Vec::new();
    let mut parsing_ingredients = false;
    let mut ranges: Vec<RangeInclusive<i64>> = Vec::new();
    for line in lines {
        if line == "" {
            parsing_ingredients = true;
        } else {
            if parsing_ingredients {
                ingredients.push(line.parse().unwrap());
            } else {
                ranges.push(parse_iclusive_range(line));
            }
        }
    }
    return (ingredients, ranges);
}

fn merge_ranges(mut ranges: Vec<RangeInclusive<i64>>) -> Vec<RangeInclusive<i64>> {
    ranges.sort_unstable_by_key(|r| *r.start());

    let mut merged_ranges: Vec<RangeInclusive<i64>> = Vec::new();
    
    let mut current_range = ranges.remove(0); 

    for next_range in ranges.into_iter() {
        if next_range.start() <= current_range.end() {
            current_range = *current_range.start()..=*max(current_range.end(), next_range.end());
        } else {
            merged_ranges.push(current_range);
            current_range = next_range;
        }
    }

    merged_ranges.push(current_range);

    merged_ranges
}

fn solve_part_1(lines: &Vec<String>) -> usize {
    let (ingredients, fresh_ranges) = get_ingredients_and_fresh_map(lines);
    return ingredients
        .iter()
        .map(|i| fresh_ranges.iter().any(|r| r.contains(i)))
        .filter(|f| *f)
        .count();
}

fn solve_part_2(lines: &Vec<String>) -> i64 {
    let (_, fresh_ranges) = get_ingredients_and_fresh_map(lines);
    let mut sum = 0;
    for range in merge_ranges(fresh_ranges) {
        sum += range.end() - range.start() + 1;
    }
    return sum;
}
