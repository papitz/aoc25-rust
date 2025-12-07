use ndarray::Array2;
use regex::Regex;
use rust_aoc::read_lines;

const DAY: u8 = 7;
const SPLITTER: char = '^';

fn main() {
    let ex_lines = read_lines(DAY, true);
    println!("Part 1 result: {}", solve_part_1(&ex_lines));
    // println!("Part 2 result: {}", solve_part_2(&ex_lines));
    // //
    let lines = read_lines(DAY, false);
    println!("Part 1 result: {}", solve_part_1(&lines));
    // println!("Part 2 result: {}", solve_part_2(&lines));
}

fn solve_part_1(lines: &Vec<String>) -> i64 {
    let mut beams: Vec<usize>;
    let (first_line, rest) = lines.split_first().unwrap();
    let beam_start: usize = first_line.find('S').unwrap();
    let mut times_split: i64 = 0;
    beams = vec![beam_start];
    for line in rest {
        if !line.contains(SPLITTER) {
            println!("No splitter in line");
            continue;
        } else {
            let splitter_indices: Vec<_> = line.match_indices(SPLITTER).map(|(i, _)| i).collect();
            let mut tmp_beams: Vec<usize> = Vec::new();
            for splitter_index in splitter_indices {
                dbg!(splitter_index);
                dbg!(&beams);
                if beams.contains(&splitter_index) {
                    println!("Splitter {splitter_index} is hit by beam");
                    tmp_beams.push(splitter_index + 1);
                    tmp_beams.push(splitter_index - 1);
                    times_split += 1;
                }
            }
            beams = tmp_beams;
        }
    }
    times_split
}

fn solve_part_2(lines: &Vec<String>) -> i64 {
    0
}
