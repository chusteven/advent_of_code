use std::collections::HashSet;

use crate::yr_2021::problems::utils;

const DIRECTIONS: &[(i32, i32); 4] = &[
    (-1, 0), // Up
    (1, 0),  // Down
    (0, 1),  // Right
    (0, -1), // Left
];

///
/// 1. Find all low points -- O(n) pass
/// 2. From low points, do graph traversal outward until get to 9s;
///     and track the size of each graph
/// 3. Heapify the size of the graphs
/// 4. Return product of top 3
///
#[allow(dead_code)]
pub fn solution_2(input_file: &str) -> i32 {
    let lines = utils::read_file(input_file).unwrap();
    let board = utils::convert_to_ints(lines);

    let rows = board.len();
    let cols = board[0].len();

    let low_points = get_low_points(&board, rows, cols);
    let mut basin_sizes = vec![];
    for (i, j) in low_points.iter() {
        let mut visited = HashSet::new();
        get_basin_size(&board, *i, *j, rows, cols, &mut visited);
        basin_sizes.push(visited.len() as i32);
    }
    basin_sizes.sort_by(|a, b| b.cmp(a));
    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
}

#[allow(dead_code)]
fn get_low_points(board: &[Vec<i32>], rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut ans = vec![];

    for (i, row) in board.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if is_low_point(board, i, j, rows, cols) {
                ans.push((i, j))
            }
        }
    }
    ans
}

#[allow(dead_code)]
fn get_basin_size(
    board: &[Vec<i32>],
    i: usize,
    j: usize,
    rows: usize,
    cols: usize,
    visited: &mut HashSet<(usize, usize)>,
) {
    visited.insert((i, j));
    for (ud, lr) in DIRECTIONS {
        let newi = (i as i32) + ud;
        let newj = (j as i32) + lr;
        if newi < 0
            || newj < 0
            || (newi as usize) == rows
            || (newj as usize) == cols
            || visited.contains(&(newi as usize, newj as usize))
            || board[newi as usize][newj as usize] == 9
        {
            continue;
        }
        get_basin_size(board, newi as usize, newj as usize, rows, cols, visited);
    }
}

#[allow(dead_code)]
pub fn solution_1(input_file: &str) -> i32 {
    let lines = utils::read_file(input_file).unwrap();
    let board = utils::convert_to_ints(lines);
    let rows = board.len();
    let cols = board[0].len();
    let mut ans = 0;
    for (i, row) in board.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if is_low_point(&board, i, j, rows, cols) {
                ans += val + 1
            }
        }
    }
    ans
}

#[allow(dead_code)]
fn is_low_point(board: &[Vec<i32>], i: usize, j: usize, rows: usize, cols: usize) -> bool {
    let cmp = board[i][j];

    let mut num_comparisons = 0;
    let mut is_lower_than = 0;

    for (ud, lr) in DIRECTIONS {
        let newi = (i as i32) + ud;
        let newj = (j as i32) + lr;
        if newi < 0 || newj < 0 || newi == (rows as i32) || newj == (cols as i32) {
            continue;
        }
        num_comparisons += 1;
        let cur = board[newi as usize][newj as usize];
        if cmp < cur {
            is_lower_than += 1;
        }
    }
    is_lower_than == num_comparisons
}
