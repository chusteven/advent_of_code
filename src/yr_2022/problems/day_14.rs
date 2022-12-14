use crate::yr_2022::problems::utils;

#[allow(dead_code)]
pub fn solution_2(input_file: &str) -> i32 {
    let lines = utils::read_file(input_file).unwrap();
    let mut board = parse_input(lines, 2);
    board[0][500] = '+';
    let mut ans = 0;
    let start_pos = (500, 0);
    loop {
        if !_add_sand_to_board(&mut board, &start_pos) {
            ans += 1;
            break;
        }
        ans += 1;
    }
    ans
}

fn _add_sand_to_board(board: &mut Vec<Vec<char>>, start_pos: &(usize, usize)) -> bool {
    let mut sand_pos = *start_pos;

    loop {
        let moved = next_sand_pos(&mut sand_pos, board);
        if !moved {
            break;
        }
    }

    // Note the new stopping condition
    if sand_pos == *start_pos {
        return false;
    }
    true
}

#[allow(dead_code)]
pub fn solution_1(input_file: &str) -> i32 {
    let lines = utils::read_file(input_file).unwrap();
    let mut board = parse_input(lines, 0);
    board[0][500] = '+';
    let mut ans = 0;
    let start_pos = (500, 0);
    loop {
        if !add_sand_to_board(&mut board, &start_pos) {
            break;
        }
        ans += 1;
    }
    ans
}

///
/// Useful to think about what the stopping condition is...
/// Probably the easiest thing is as soon as we try and send sand
/// into a (x, y) that we've already tried to send sand into
/// before?
///
/// NOTE: That wasn't a great stopping condition -- instead
/// we just check to see when a piece of sand has gotten to the
/// same level as the floor
///
fn add_sand_to_board(board: &mut Vec<Vec<char>>, start_pos: &(usize, usize)) -> bool {
    // Send sand to where it should go
    let mut sand_pos = *start_pos;

    // A unit of sand always falls down one step if possible.
    // If the tile immediately below is blocked (by rock or sand),
    // the unit of sand attempts to instead move diagonally one
    // step down and to the left. If that tile is blocked, the
    // unit of sand attempts to instead move diagonally one step
    // down and to the right. Sand keeps moving as long as it is
    // able to do so, at each step trying to move down, then
    // down-left, then down-right. If all three possible destinations
    // are blocked, the unit of sand comes to rest and no longer
    // moves, at which point the next unit of sand is created back
    // at the source.

    loop {
        let moved = next_sand_pos(&mut sand_pos, board);
        if !moved {
            break;
        }
    }

    if sand_pos.1 == board.len() - 1 {
        return false;
    }
    true
}

fn next_sand_pos(sand_pos: &mut (usize, usize), board: &mut Vec<Vec<char>>) -> bool {
    let (x, y) = *sand_pos;
    if y + 1 != board.len() && board[y + 1][x] == '.' {
        // Try and go down
        *sand_pos = (x, y + 1);
        return true;
    }
    if x > 0 && y + 1 != board.len() && board[y + 1][x - 1] == '.' {
        // Try to go down and left
        *sand_pos = (x - 1, y + 1);
        return true;
    }
    if x + 1 != board[0].len() && y + 1 != board.len() && board[y + 1][x + 1] == '.' {
        // Down and right
        *sand_pos = (x + 1, y + 1);
        return true;
    }
    board[y][x] = 'o';
    false
}

fn parse_input(lines: Vec<String>, extra_floor: usize) -> Vec<Vec<char>> {
    let mut max_x = 0;
    let mut max_y = 0;
    for line in &lines {
        let parts = line
            .split(" -> ")
            .into_iter()
            .map(|c| {
                let mut parts = c.split(',');
                let x = parts.next().unwrap().parse::<usize>().unwrap();
                let y = parts.next().unwrap().parse::<usize>().unwrap();
                (x, y)
            })
            .collect::<Vec<(usize, usize)>>();
        for (x, y) in parts {
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
        }
    }

    max_x *= 2; // FIXME: This seems kinda like a hack

    // Note the +1 so that indexing is easier
    let mut board = vec![vec!['.'; max_x + 1]; max_y + 1 + extra_floor];
    for line in &lines {
        let parts = line
            .split(" -> ")
            .into_iter()
            .map(|c| {
                let mut parts = c.split(',');
                let x = parts.next().unwrap().parse::<usize>().unwrap();
                let y = parts.next().unwrap().parse::<usize>().unwrap();
                (x, y)
            })
            .collect::<Vec<(usize, usize)>>();
        for i in 1..parts.len() {
            let (prv_x, prv_y) = parts[i - 1];
            let (cur_x, cur_y) = parts[i];
            if prv_x == cur_x {
                // Same column, so go up/down
                let lower = if prv_y < cur_y { prv_y } else { cur_y };
                let upper = if prv_y > cur_y { prv_y } else { cur_y };

                for board_item in board.iter_mut().take(upper + 1).skip(lower) {
                    board_item[prv_x] = '#';
                }
            } else {
                // Same row, so go left/right
                let lower = if prv_x < cur_x { prv_x } else { cur_x };
                let upper = if prv_x > cur_x { prv_x } else { cur_x };
                for j in lower..(upper + 1) {
                    board[prv_y][j] = '#';
                }
            }
        }
    }

    if extra_floor > 0 {
        for i in 0..max_x {
            board[max_y + extra_floor][i] = '#';
        }
    }
    board
}
