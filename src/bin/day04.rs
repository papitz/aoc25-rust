use ndarray::Array2;
use rust_aoc::{get_neighbor_indices, read_lines};

const DAY: u8 = 4;

fn main() {
    // dbg!(get_neighbor_indices(0, 0, 7, 8));
    let ex_lines = read_lines(DAY, true);
    // let mx = build_paper_matrix(&ex_lines);
    // dbg!("{}", mx.shape());
    println!("Part 1 result: {}", solve_part_1(&ex_lines));
    println!("Part 2 result: {}", solve_part_2(&ex_lines));
    //
    let lines = read_lines(DAY, false);
    println!("Part 1 result: {}", solve_part_1(&lines));
    println!("Part 2 result: {}", solve_part_2(&lines));
}

fn build_paper_matrix(lines: &Vec<String>) -> Array2<bool> {
    let rows = lines.len();
    let cols = lines[0].len();
    let mut data1d = Vec::new();
    for line in lines {
        let has_roll = line.chars().map(|c| c == '@');
        data1d.extend(has_roll);
    }
    let matrix = Array2::from_shape_vec((rows, cols), data1d).expect("Smth went wrong");
    return matrix;
}

fn check_less_than_nbs(
    matrix: &Array2<bool>,
    max_neigbors: usize,
    (index_row, index_col): (usize, usize),
    (max_row, max_col): (usize, usize),
) -> bool {
    let nb_indices = get_neighbor_indices(index_row, index_col, max_row, max_col);
    nb_indices
        .map(|nb_index| matrix[nb_index])
        .filter(|has_paper| *has_paper)
        .collect::<Vec<_>>()
        .len()
        <= max_neigbors
}

fn solve_part_1(lines: &Vec<String>) -> i64 {
    let mut sum = 0;
    let max_neighbors = 3;
    let matrix = build_paper_matrix(lines);
    let &[max_row, max_col]: &[usize; 2] = matrix
        .shape()
        .try_into()
        .expect("Array must be 2-dimensional");
    let shape = (max_row, max_col);
    for row in 0..max_row {
        for col in 0..max_col {
            if matrix[[row, col]] {
                if check_less_than_nbs(&matrix, max_neighbors, (row, col), shape) {
                    sum += 1;
                }
            }
        }
    }
    return sum;
}

fn calculate_removed_and_new_matrix(
    max_neighbors: usize,
    matrix: Array2<bool>,
    max_row: usize,
    max_col: usize,
) -> (i64, Array2<bool>) {
    let mut sum: i64 = 0;
    let mut new_elems: Vec<bool> = Vec::new();
    let shape = (max_row, max_col);
    for row in 0..max_row {
        for col in 0..max_col {
            let mut has_paper = matrix[[row, col]];
            if has_paper {
                if check_less_than_nbs(&matrix, max_neighbors, (row, col), shape) {
                    sum += 1;
                    has_paper = false;
                }
            }
            new_elems.push(has_paper);
        }
    }
    (
        sum,
        Array2::from_shape_vec((max_row, max_col), new_elems).expect("Shape worked"),
    )
}

fn solve_part_2(lines: &Vec<String>) -> i64 {
    let mut sum = 0;
    let max_neighbors = 3;
    let mut matrix = build_paper_matrix(lines);
    let &[max_row, max_col]: &[usize; 2] = matrix
        .shape()
        .try_into()
        .expect("Array must be 2-dimensional");
    loop {
        let removed_papers;
        (removed_papers, matrix) =
            calculate_removed_and_new_matrix(max_neighbors, matrix, max_row, max_col);
        sum += removed_papers;
        if removed_papers == 0 {
            break;
        }
    }
    return sum;
}
