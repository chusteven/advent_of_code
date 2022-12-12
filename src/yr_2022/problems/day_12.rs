use std::collections::{HashSet, VecDeque};

use crate::yr_2022::problems::utils;

const DIRECTIONS: &[(i32, i32); 4] = &[
    (-1, 0), // Up
    (1, 0),  // Down
    (0, 1),  // Right
    (0, -1), // Left
];

#[derive(Debug)]
struct State {
    pos: (i32, i32),
    steps: i32,
}

#[allow(dead_code)]
pub fn solution_2(input_file: &str) -> i32 {
    let lines = utils::read_file(input_file).unwrap();
    let mut start = (0, 0);
    let mut starting_points = vec![];
    let mut end = (0, 0);
    let mut grid = lines
        .iter()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .into_iter()
                .enumerate()
                .map(|(j, c)| {
                    if c == 'S' {
                        start = (i as i32, j as i32);
                        starting_points.push((i as i32, j as i32));
                    } else if c == 'a' {
                        starting_points.push((i as i32, j as i32));
                    } else if c == 'E' {
                        end = (i as i32, j as i32);
                    }
                    (c as i32) - 96
                }) // Probably not useful :D
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    grid[start.0 as usize][start.1 as usize] = 1;
    grid[end.0 as usize][end.1 as usize] = 26;
    let mut ans = std::i32::MAX;
    for sp in starting_points {
        let visited = HashSet::new();
        let queue = VecDeque::new();
        let cur = bfs(sp, &end, &grid, visited, queue);
        if cur < ans {
            ans = cur;
        }
    }
    ans
}

///
/// BFS I think gets us there fastest
///
#[allow(dead_code)]
pub fn solution_1(input_file: &str) -> i32 {
    let lines = utils::read_file(input_file).unwrap();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut grid = lines
        .iter()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .into_iter()
                .enumerate()
                .map(|(j, c)| {
                    if c == 'S' {
                        start = (i as i32, j as i32);
                    } else if c == 'E' {
                        end = (i as i32, j as i32);
                    }
                    (c as i32) - 96
                }) // Probably not useful :D
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    grid[start.0 as usize][start.1 as usize] = 1;
    grid[end.0 as usize][end.1 as usize] = 26;
    let visited = HashSet::new();
    let queue = VecDeque::new();
    bfs(start, &end, &grid, visited, queue)
}

fn bfs(
    start: (i32, i32),
    end: &(i32, i32),
    grid: &Vec<Vec<i32>>,
    mut visited: HashSet<(i32, i32)>,
    mut queue: VecDeque<State>,
) -> i32 {
    visited.insert(start);
    queue.push_back(State {
        pos: start,
        steps: 0,
    });
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        let m = node.pos;
        let cur_v = grid[m.0 as usize][m.1 as usize];
        for dir in DIRECTIONS {
            let next = (m.0 + dir.0, m.1 + dir.1);
            if next.0 < 0
                || next.1 < 0
                || next.0 >= (grid.len() as i32)
                || next.1 >= (grid[0].len() as i32)
            {
                continue;
            }
            let next_v = grid[next.0 as usize][next.1 as usize];
            if next_v - cur_v > 1 {
                continue;
            }
            if next == *end {
                return node.steps + 1;
            }
            if !visited.contains(&next) {
                visited.insert(next);
                queue.push_back(State {
                    pos: next,
                    steps: node.steps + 1,
                });
            }
        }
    }
    std::i32::MAX // Huh! This means we never found the end via this starting point
}
