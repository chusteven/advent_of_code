use std::collections::VecDeque;

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
    for _line in lines {
        //
    }
    0
}

#[allow(dead_code)]
pub fn solution_1(input_file: &str) -> i32 {
    let lines = utils::read_file(input_file).unwrap();
    let (sensors, beacons) = parse_input(lines);
    // println!("{:?}", sensors);
    // println!("{:?}", beacons);

    let mut max_x = 0;
    let mut max_y = 0;
    sensors.iter().for_each(|(x, y)| {
        if *x > max_x {
            max_x = *x;
        }
        if *y > max_y {
            max_y = *y;
        }
    });
    beacons.iter().for_each(|(x, y)| {
        if *x > max_x {
            max_x = *x;
        }
        if *y > max_y {
            max_y = *y;
        }
    });

    let mut max_x = max_x as usize;
    let mut max_y = max_y as usize;

    // For testing only
    // max_x = 2;
    // max_y = 4;

    // We may go into -x territory as well as -y territory --
    // this means we should find that offset (for each axis)
    // and we that to index into the final grid

    // Doubling to be conservative
    let mut grid = vec![vec!['.'; 2 * max_x + 1]; 2 * max_y + 1];

    // // For testing only
    // // If I wanna put something at (0, 0)
    // grid[max_y + 0][max_x + 0] = 'x';
    // // If I wanna put something at (2, 0)
    // grid[max_y + 0][max_x + max_x] = 'x';
    // // If I wanna put something at (2, 4)
    // grid[max_y + max_y][max_x + max_x] = 'x';
    // println!("{:?}", grid);

    for (s, b) in sensors.iter().zip(beacons.iter()) {
        mark_grid(s, b, &mut grid, &max_x, &max_y)
    }
    let row_of_interest = 2_000_000;
    let mut ans = 0;
    for i in 0..grid[row_of_interest].len() {
        if grid[row_of_interest][i] == '#' {
            ans += 1;
        }
    }
    ans
}

fn mark_grid(
    sensor: &(i32, i32),
    beacon: &(i32, i32),
    grid: &mut Vec<Vec<char>>,
    max_x: &usize,
    max_y: &usize,
) {
    // BFS outward; stopping condition is when Manhattan distance is farther
    // than it is now

    let mut queue: VecDeque<State> = VecDeque::new();
    queue.push_back(State {
        pos: *sensor,
        steps: 0,
    });
    let start = *sensor;
    let manhattan_dist = (beacon.1 - sensor.1).abs() + (beacon.0 - sensor.0).abs();
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        let m = node.pos;
        for dir in DIRECTIONS {
            let next = (m.0 + dir.0, m.1 + dir.1);
            if (max_y + next.1 as usize) >= grid.len() || (max_x + next.0 as usize) >= grid[0].len()
            {
                continue;
            }
            let next_v = grid[max_y + next.0 as usize][max_x + next.1 as usize];
            if next_v == 'S' || next_v == 'B' || next_v == '#' {
                continue;
            }
            if (next.1 - start.1).abs() + (next.0 - start.0).abs() > manhattan_dist {
                continue;
            }
            grid[max_y + (next.1 as usize)][max_x + (next.0 as usize)] = '#';
            queue.push_back(State {
                pos: next,
                steps: node.steps + 1,
            });
        }
    }
}

fn parse_input(lines: Vec<String>) -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let mut sensors = vec![];
    let mut beacons = vec![];

    for line in lines {
        let parts = line.split(':');
        let mut sensor = (0, 0);
        let mut beacon = (0, 0);
        for (i, p) in parts.into_iter().enumerate() {
            if i == 0 {
                sensor = get_coordinates_from_str(p);
            } else {
                beacon = get_coordinates_from_str(p);
            }
        }
        sensors.push(sensor);
        beacons.push(beacon);
    }

    (sensors, beacons)
}

fn get_coordinates_from_str(string: &str) -> (i32, i32) {
    let parts = string.split(' ');
    let mut x = 0;
    let mut y = 0;
    for p in parts {
        let n = p.len();
        if p.contains("x=") {
            let ind = p.find("x=").unwrap();
            x = p[ind + 2..n - 1].trim().parse::<i32>().unwrap();
        } else if p.contains("y=") {
            let ind = p.find("y=").unwrap();
            y = p[ind + 2..n].trim().parse::<i32>().unwrap();
        }
    }
    (x, y)
}
