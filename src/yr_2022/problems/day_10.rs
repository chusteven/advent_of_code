use crate::yr_2022::problems::utils;

#[allow(dead_code)]
pub fn solution_2(input_file: &str) -> i32 {
    let lines = utils::read_file(input_file).unwrap();

    // Init drawing
    let mut drawing = vec![];
    for _ in 0..6 {
        let v = vec!["."; 40];
        drawing.push(v);
    }

    let mut x = 1;
    let mut cur_cycle = 0;
    for (_i, line) in lines.iter().enumerate() {
        if line.starts_with("noop") {
            maybe_draw_lit(&mut drawing, cur_cycle / 40, cur_cycle % 40, x);
            cur_cycle += 1;
            continue;
        }
        let parts: Vec<&str> = line.split(' ').collect();
        let v = parts[1].trim().parse::<i32>().unwrap();
        maybe_draw_lit(&mut drawing, cur_cycle / 40, cur_cycle % 40, x);
        cur_cycle += 1;
        maybe_draw_lit(&mut drawing, cur_cycle / 40, cur_cycle % 40, x);
        cur_cycle += 1;
        x += v; // Do this last b/c we report on the state DURING the cycle
    }

    for r in drawing {
        for x in r {
            print!("{x}");
        }
        println!();
    }
    0
}

fn maybe_draw_lit(drawing: &mut Vec<Vec<&str>>, row: usize, col: usize, x: i32) {
    if (col as i32) >= x - 1 && (col as i32) <= x + 1 {
        (*drawing)[row][col] = "#";
    }
}

#[allow(dead_code)]
pub fn solution_1(input_file: &str) -> i32 {
    let lines = utils::read_file(input_file).unwrap();
    let mut x = 1;
    let mut ans = 0;
    let mut cur_cycle = 0;
    for (_i, line) in lines.iter().enumerate() {
        if line.starts_with("noop") {
            cur_cycle += 1;
            if cur_cycle == 20 || (cur_cycle >= 60 && ((cur_cycle - 20) % 40 == 0)) {
                ans += cur_cycle * x;
            }
            continue;
        }
        let parts: Vec<&str> = line.split(' ').collect();
        let v = parts[1].trim().parse::<i32>().unwrap();
        cur_cycle += 1;
        if cur_cycle == 20 || (cur_cycle >= 60 && ((cur_cycle - 20) % 40 == 0)) {
            ans += cur_cycle * x;
        }
        cur_cycle += 1;
        if cur_cycle == 20 || (cur_cycle >= 60 && ((cur_cycle - 20) % 40 == 0)) {
            ans += cur_cycle * x;
        }
        x += v; // Do this last b/c we report on the state DURING the cycle
    }
    ans
}
