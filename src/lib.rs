use std::fs;

// 'pub' makes this function available to your binaries
pub fn read_lines(day: u8, example_data: bool) -> Vec<String> {
    // 1. Format the filename.
    // {:02} pads the number with zeros (e.g., 1 becomes "01")
    let prefix = if example_data { "ex_" } else { "" };

    let file_path = format!("./input/{day}/{prefix}input.txt");

    // 2. Read the file
    let contents = fs::read_to_string(&file_path).expect(&format!(
        "Should have been able to read the file: {}",
        file_path
    ));

    // 3. Convert to a list of strings
    contents.lines().map(|line| line.to_string()).collect()
}

pub fn read_csv_into_lines(day: u8, example_data: bool) -> Vec<String> {
    let csv_line = &read_lines(day, example_data)[0];
    return csv_line.split(',').map(|s| s.trim().to_string()).collect();
}

pub fn get_neighbor_indices(
    row: usize,
    col: usize,
    max_row: usize,
    max_col: usize,
) -> impl Iterator<Item = (usize, usize)> {
    [-1, 0, 1].into_iter().flat_map(move |dx| {
        [-1, 0, 1].into_iter().filter_map(move |dy| {
            (dx != 0 || dy != 0)
                .then_some((row.checked_add_signed(dx)?, col.checked_add_signed(dy)?))
                .filter(|&(row, col)| row < max_row && col < max_col)
        })
    })
}
