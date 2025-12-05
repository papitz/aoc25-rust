use rust_aoc::read_lines;

const DIAL_SIZE: i32 = 100;

fn main() {
    let ex_lines = read_lines(1, true);

    println!("Part 1 result: {}", solve_part_1(&ex_lines));
    println!("Part 2 result: {}", solve_part_2(&ex_lines));

    let lines = read_lines(1, false);

    println!("Part 1 result: {}", solve_part_1(&lines));
    println!("Part 2 result: {}", solve_part_2(&lines));
}

fn parse_input(input: &str) -> i32 {
    let sign = match input.chars().next() {
        Some('L') => -1,
        _ => 1,
    };
    let rotation_str = &input[1..];
    let rotation = rotation_str.parse::<i32>().unwrap();
    return sign * rotation;
}

fn put_in_rotation(current_position: i32, rotation: i32) -> i32 {
    return (current_position + rotation).rem_euclid(DIAL_SIZE);
}

fn count_zero_passes(current_position: i32, rotation: i32) -> (i32, i32) {
    let end_position = put_in_rotation(current_position, rotation);
    let mut zero_count = 0;
    if rotation < 0 {
        if current_position == 0 {
            zero_count += rotation.abs() / DIAL_SIZE
        } else if rotation.abs() > current_position {
            let remaining_rotation = rotation.abs() - current_position - 1;
            zero_count += (remaining_rotation / DIAL_SIZE) + 1;
        }
        if end_position == 0 {
            zero_count += 1;
        }
    } else {
        zero_count += (rotation + current_position) / DIAL_SIZE
    }
    return (end_position, zero_count);
}

fn solve_part_1(lines: &Vec<String>) -> i32 {
    let mut nums: Vec<i32> = Vec::new();
    let mut current_position = 50;
    for input in lines {
        let rotation = parse_input(input);
        current_position = put_in_rotation(current_position, rotation);
        nums.push(current_position);
    }

    return nums.iter().filter(|&num| *num == 0).count() as i32;
}

fn solve_part_2(lines: &Vec<String>) -> i32 {
    let mut total_zero_passes = 0;
    let mut current_position = 50;
    for input in lines {
        let rotation = parse_input(input);
        let zero_passes: i32;
        (current_position, zero_passes) = count_zero_passes(current_position, rotation);
        total_zero_passes += zero_passes
    };
    return total_zero_passes;
}
