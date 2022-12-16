use std::collections::VecDeque;

use crate::yr_2022::problems::utils;

const DIRECTIONS: &[(i32, i32); 4] = &[
    (-1, 0), // Up
    (1, 0),  // Down
    (0, 1),  // Right
    (0, -1), // Left
];

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

    let mut offset_x = 0;
    let mut offset_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    sensors.iter().zip(beacons.iter()).for_each(|(s, b)| {
        if s.0 > max_x {
            max_x = s.0
        }
        if b.0 > max_x {
            max_x = b.0
        }
        if s.1 > max_y {
            max_y = s.1
        }
        if b.1 > max_y {
            max_y = b.1
        }
        if (s.0 - b.0).abs() > offset_x {
            offset_x = (s.0 - b.0).abs();
        }
        if (s.1 - b.1).abs() > offset_y {
            offset_y = (s.1 - b.1).abs();
        }
    });

    // We may go into -x territory as well as -y territory --
    // this means we should find that offset (for each axis)
    // and we that to index into the final grid
    //
    // Doubling to be conservative (probably too much...)
    let offset_x = offset_x as usize;
    let offset_y = offset_y as usize;
    let max_x = max_x as usize;
    let max_y = max_y as usize;

    // This is what's taking forever :(
    println!("Filling -- ({offset_x}, {offset_y})");
    let mut grid = vec![vec!['.'; offset_x + max_x + 1]; offset_y + max_y + 1];
    println!("Filled");

    println!("Marking");
    for (s, b) in sensors.iter().zip(beacons.iter()) {
        grid[(offset_y as i32 + s.1) as usize][(offset_x as i32 + s.0) as usize] = 'S';
        grid[(offset_y as i32 + b.1) as usize][(offset_x as i32 + b.0) as usize] = 'B';
    }
    println!("Marked");

    for (s, b) in sensors.iter().zip(beacons.iter()) {
        mark_grid(s, b, &mut grid, &offset_x, &offset_y)
    }
    let row_of_interest = if max_y > 2_000_000 { 2_000_000 } else { 10 };
    let mut ans = 0;
    for i in 0..grid[offset_y + row_of_interest].len() {
        let val = grid[offset_y + row_of_interest][i];
        if val == '#' || val == 'S' || val == 'B' {
            ans += 1;
        }
    }
    ans
}

fn mark_grid(
    sensor: &(i32, i32),
    beacon: &(i32, i32),
    grid: &mut Vec<Vec<char>>,
    offset_x: &usize,
    offset_y: &usize,
) {
    // BFS outward; stopping condition is when Manhattan distance is farther
    // than it is now
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((
        (*offset_x as i32 + sensor.0) as usize,
        (*offset_y as i32 + sensor.1) as usize,
    ));
    let start = (sensor.0 + *offset_x as i32, sensor.1 + *offset_y as i32);
    let manhattan_dist = (beacon.1 - sensor.1).abs() + (beacon.0 - sensor.0).abs();
    while !queue.is_empty() {
        let m = queue.pop_front().unwrap();
        for dir in DIRECTIONS {
            let next = (m.0 as i32 + dir.0, m.1 as i32 + dir.1);
            if next.0 < 0
                || next.1 < 0
                || (next.1 as usize) >= grid.len()
                || (next.0 as usize) >= grid[0].len()
            {
                continue;
            }
            let next = (next.0 as usize, next.1 as usize);
            let next_v = grid[next.1][next.0];
            if next_v == 'S'
                || next_v == 'B'
                || next_v == '#'
                || (next.1 as i32 - start.1).abs() + (next.0 as i32 - start.0).abs()
                    > manhattan_dist
            {
                continue;
            } else {
                grid[next.1][next.0] = '#';
            }
            queue.push_back(next);
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
