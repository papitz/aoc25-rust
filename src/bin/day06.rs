use ndarray::Array2;
use regex::Regex;
use rust_aoc::read_lines;

const DAY: u8 = 6;

fn main() {
    let ex_lines = read_lines(DAY, true);
    println!("Part 1 result: {}", solve_part_1(&ex_lines));
    println!("Part 2 result: {}", solve_part_2(&ex_lines));
    //
    let lines = read_lines(DAY, false);
    println!("Part 1 result: {}", solve_part_1(&lines));
    println!("Part 2 result: {}", solve_part_2(&lines));
}

fn compress_space_into_commas(line: &String) -> String {
    let re = Regex::new(r"\ +").unwrap();
    re.replace_all(&line.trim(), ",").to_string()
}

fn remove_space(line: &String) -> String {
    let re = Regex::new(r"\ +").unwrap();
    re.replace_all(&line, "").to_string()
}

fn parse_lines_into_col_nums(lines: &Vec<String>, number_of_problems: usize, neutrals: &Vec<i64>) -> Array2<i64> {
    let line_len = lines.first().unwrap().len();
    let rows = lines.len();
    let mut num_strings: Vec<String> = vec![String::new(); line_len];
    for line in lines {
        for (index, c) in line.chars().rev().enumerate() {
            num_strings[index].push(c);
        }
    }
    let mut nums: Array2<i64> = Array2::zeros((number_of_problems, rows));
    for i in 0..number_of_problems {
        let neutral_for_problem = neutrals[i];
        for j in 0..rows {
            nums[[i, j]] = neutral_for_problem;
        }
    }

    let mut current_problem = 0;
    let mut current_num = 0;
    for num_str in num_strings {
        let stripped_str = remove_space(&num_str);
        if stripped_str.is_empty() {
            current_problem += 1;
            current_num = 0;
        } else {
            nums[[current_problem, current_num]] = stripped_str.parse().unwrap();
            current_num += 1;
        }
    }
    nums
}

fn calculate(current_res: i64, operator: char, input: i64) -> i64 {
    match operator {
        '*' => current_res * input,
        '+' => current_res + input,
        _ => panic!("Operator not supported"),
    }
}

fn solve_part_1(lines: &Vec<String>) -> i64 {
    let (operator_str, rem_lines) = lines.split_last().unwrap();
    let operators: Vec<char> = remove_space(operator_str).chars().collect();
    let mut results: Vec<i64> = operators
        .iter()
        .map(|c| if *c == '*' { 1 } else { 0 })
        .collect();
    for line in rem_lines {
        for (index, num) in compress_space_into_commas(line).split(',').enumerate() {
            results[index] = calculate(results[index], operators[index], num.parse().unwrap());
        }
    }
    results.iter().sum()
}

fn solve_part_2(lines: &Vec<String>) -> i64 {
    let (operator_str, rem_lines) = lines.split_last().unwrap();
    let operators: Vec<char> = remove_space(operator_str).chars().rev().collect();
    let mut results: Vec<i64> = operators
        .iter()
        .map(|c| if *c == '*' { 1 } else { 0 })
        .collect();
    let problems = parse_lines_into_col_nums(&rem_lines.to_vec(), operators.len(), &results);
    println!("{problems}");
    let (problem_amount, nums_per_problem) = problems.dim();
    for p_i in 0..problem_amount {
        for n_i in 0..nums_per_problem {
            results[p_i] = calculate(results[p_i], operators[p_i], problems[[p_i, n_i]])
        }
    }
    results.iter().sum()
}
