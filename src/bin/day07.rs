use std::{
    collections::{HashMap, HashSet},
    iter::repeat,
};

use rust_aoc::read_lines;

const DAY: u8 = 7;
const SPLITTER: char = '^';

#[derive(Debug)]
struct Node {
    level: usize,
    width: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn with_children(
        level: usize,
        width: usize,
        left: Option<Node>,
        right: Option<Node>,
    ) -> Self {
        Node {
            level: level,
            width: width,
            left: left.map(Box::new),
            right: right.map(Box::new),
        }
    }

    pub fn print_tree(&self) {
        let str_width = self.width * 2 + 1;
        let empty_line: String = repeat('.').take(str_width).collect();
        let mut lines: HashMap<usize, String> = HashMap::new();
        let mut next_nodes: Vec<&Node> = Vec::new();
        next_nodes.push(self);
        while !next_nodes.is_empty() {
            let next = next_nodes.remove(0);
            if !lines.contains_key(&next.level) {
                lines.insert(next.level, empty_line.to_string());
            }
            let mut line_str: String = lines.get(&next.level).unwrap().to_string();

            line_str.replace_range(next.width..=next.width, "^");
            lines.insert(next.level, line_str);

            match next.left.as_ref() {
                Some(node) => next_nodes.push(&node),
                None => {}
            }
            match next.right.as_ref() {
                Some(node) => next_nodes.push(&node),
                None => {}
            }
        }
        let mut sorted_keys: Vec<usize> = lines.keys().cloned().collect();
        sorted_keys.sort();
        for key in sorted_keys.iter().rev() {
            println!("{}", lines[&key]);
        }
    }

    pub fn count_sub_nodes(&self) -> i64 {
        let mut count = 1;

        // Recursively count nodes in the left subtree
        // If 'self.left' is Some(box_node), map transforms the Box<Node<T>>
        // into a usize by calling count_nodes() on the inner node.
        // If it's None, it uses the default value of 0.
        count += self
            .left
            .as_ref()
            .map_or(0, |boxed_node| boxed_node.count_sub_nodes());

        // Recursively count nodes in the right subtree
        count += self
            .right
            .as_ref()
            .map_or(0, |boxed_node| boxed_node.count_sub_nodes());

        count
    }
}

fn main() {
    let ex_lines = read_lines(DAY, true);
    // build_tree(&ex_lines).print_tree();
    // println!("Part 1 result: {}", solve_part_1(&ex_lines));
    println!("Part 2 result: {}", solve_part_2(&ex_lines));
    // //
    let lines = read_lines(DAY, false);
    // build_tree(&lines).print_tree();
    // println!("Part 1 result: {}", solve_part_1(&lines));
    println!("Part 2 result: {}", solve_part_2(&lines));
}
//
// fn build_tree(lines: &Vec<String>) -> Node {
//     let lines = lines.iter().rev().filter(|line| line.contains(SPLITTER));
//     let mut current_nodes: HashMap<usize, Node> = HashMap::new();
//     for (level, line) in lines.enumerate() {
//         let splitter_indices: Vec<_> = line.match_indices(SPLITTER).map(|(i, _)| i).collect();
//         let new_nodes: HashMap<usize, Node> = splitter_indices
//             .into_iter()
//             .map(|width| {
//                 (
//                     width,
//                     Node::with_children(
//                         level,
//                         width,
//                         current_nodes.get(&(width - 1)),
//                         current_nodes.get(&(width + 1)),
//                     ),
//                 )
//             })
//             .collect();
//         current_nodes.extend(new_nodes);
//     }
//     let (_, last_node) = current_nodes.drain().next().unwrap();
//     last_node
// }

fn solve_part_1(lines: &Vec<String>) -> i64 {
    // let start_node = build_tree(lines);
    // start_node.count_sub_nodes()
    // 0
    let mut beams: HashSet<usize> = HashSet::new();
    let (first_line, rest) = lines.split_first().unwrap();
    let mut new_lines: Vec<String> = Vec::new();
    new_lines.push(first_line.to_string());
    let beam_start: usize = first_line.find('S').unwrap();
    let mut times_split: i64 = 0;
    beams.insert(beam_start);
    for line in rest {
        if line.contains(SPLITTER) {
            let splitter_indices: Vec<_> = line.match_indices(SPLITTER).map(|(i, _)| i).collect();
            let mut tmp_beams: HashSet<usize> = HashSet::new();
            println!("{:?}", &splitter_indices);
            for splitter_index in splitter_indices {
                if beams.contains(&splitter_index) {
                    beams.remove(&splitter_index);
                    dbg!(&beams);
                    tmp_beams.insert(splitter_index + 1);
                    tmp_beams.insert(splitter_index - 1);
                    times_split += 1;
                }
            }
            beams.extend(tmp_beams);
        }
        let line_w_beams: String = line
            .chars()
            .enumerate()
            .map(|(i, c)| if beams.contains(&i) { '|' } else { c })
            .collect();
        new_lines.push(line_w_beams);
    }
    println!("{:#?}", new_lines);
    times_split
}

fn solve_part_2(lines: &Vec<String>) -> usize {
    let (first_line, _) = lines.split_first().unwrap();
    let beam_start: usize = first_line.find('S').unwrap();

    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();

    count_beams_recursive(1, beam_start, lines, &mut memo)
}

fn count_beams_recursive(
    row: usize,
    col: usize,
    lines: &Vec<String>,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if row >= lines.len() {
        return 1;
    }

    if let Some(&count) = memo.get(&(row, col)) {
        return count;
    }

    let current_line = &lines[row];

    let result = if current_line[col..].starts_with(SPLITTER) {
        let left_res = count_beams_recursive(row + 1, col - 1, lines, memo);

        let right_res = count_beams_recursive(row + 1, col + 1, lines, memo);

        left_res + right_res
    } else {
        // No Splitter: Continue straight down
        count_beams_recursive(row + 1, col, lines, memo)
    };

    memo.insert((row, col), result);

    result
}
