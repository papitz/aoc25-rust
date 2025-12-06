use rust_aoc::read_lines;

const DAY: u8 = 3;

fn main() {
    let ex_lines = read_lines(DAY, true);
    println!("Part 1 result: {}", solve_part_1(&ex_lines));
    println!("Part 2 result: {}", solve_part_2(&ex_lines));

    let lines = read_lines(DAY, false);
    println!("Part 1 result: {}", solve_part_1(&lines));
    println!("Part 2 result: {}", solve_part_2(&lines));
}

fn get_max_and_index_from_chars(num_chars: &[char]) -> (&char, usize) {
    let max = num_chars.iter().max().unwrap();
    let index = num_chars.iter().position(|c| c == max).unwrap();
    return (max, index);
}

fn get_highest_num_and_split(num_chars: &[char], start_index: usize, remaining: usize) -> String {
    // println!("{}", num_chars.iter().collect::<String>());
    // println!("Called with from: {start_index} and rem: {remaining}");
    if num_chars.len() == remaining {
        return num_chars.iter().collect();
    }
    if num_chars.len() <= 1 {
        // println!("num_chars < 1");
        return num_chars.first().unwrap().to_string();
    }
    let (_, new_source) = num_chars.split_at(start_index);
    if new_source.len() <= 1 {
        // println!("new source < 1");
        return new_source.first().unwrap().to_string();
    }
    if remaining <= 1 {
        let (max, _) = get_max_and_index_from_chars(new_source);
        return max.to_string();
    }

    let (current_source, _) = new_source
        .split_at_checked(new_source.len() - (remaining - 1))
        .unwrap();
    let (max, max_index) = get_max_and_index_from_chars(current_source);
    let num_str = max.to_string();
    return num_str + &get_highest_num_and_split(new_source, max_index + 1, remaining - 1);
}

fn get_highest_two_digit_number(num_str: &String) -> i64 {
    get_highest_num_for_digits(num_str, 2)
}

fn get_highest_num_for_digits(num_str: &String, digits: usize) -> i64 {
    let chars: Vec<char> = num_str.chars().collect();
    let highest_possible_num = get_highest_num_and_split(&chars, 0, digits);
    return highest_possible_num.parse().unwrap();
}

fn solve_part_1(lines: &Vec<String>) -> i64 {
    let mut sum = 0;
    let mut line = 1;
    for input in lines {
        let num_from_input = get_highest_two_digit_number(input);
        println!("{line}: {num_from_input}");
        sum += num_from_input;
        line += 1;
    }
    return sum;
}

fn solve_part_2(lines: &Vec<String>) -> i64 {
    let mut sum: i64 = 0;
    let mut line = 1;
    for input in lines {
        let num_from_input = get_highest_num_for_digits(input, 12);
        println!("{line}: {num_from_input}");
        sum += num_from_input;
        line += 1;
    }
    return sum;
}
