use std::cmp::Ordering;

use crate::yr_2022::problems::utils;

pub fn solution_2(filepath: &str) -> i32 {
    let lines: Vec<Vec<i32>> = utils::read_file(filepath)
        .unwrap()
        .iter()
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let rows = lines.len();
    let cols = lines[0].len();
    let mut ans = 0;
    for i in 0..rows {
        for j in 0..cols {
            let scenic_score = get_scenic_score(i as i32, j as i32, &lines);
            if scenic_score > ans {
                ans = scenic_score;
            }
        }
    }
    ans
}

fn get_scenic_score(i: i32, j: i32, lines: &Vec<Vec<i32>>) -> i32 {
    let cmp = lines[i as usize][j as usize];
    let mut ans = 1;

    let mut _i = i - 1;
    let mut view = 0;
    while _i > -1 {
        let cur = lines[_i as usize][j as usize];
        match cur.cmp(&cmp) {
            Ordering::Less => view += 1,
            _ => {
                view += 1;
                break;
            }
        }
        _i -= 1
    }
    ans *= view;

    let mut _i = i + 1;
    let mut view = 0;
    while _i < (lines.len() as i32) {
        let cur = lines[_i as usize][j as usize];
        match cur.cmp(&cmp) {
            Ordering::Less => view += 1,
            _ => {
                view += 1;
                break;
            }
        }
        _i += 1
    }
    ans *= view;

    let mut _j = j - 1;
    let mut view = 0;
    while _j > -1 {
        let cur = lines[i as usize][_j as usize];
        match cur.cmp(&cmp) {
            Ordering::Less => view += 1,
            _ => {
                view += 1;
                break;
            }
        }
        _j -= 1
    }
    ans *= view;

    let mut _j = j + 1;
    let mut view = 0;
    while _j < (lines[0].len() as i32) {
        let cur = lines[i as usize][_j as usize];
        match cur.cmp(&cmp) {
            Ordering::Less => view += 1,
            _ => {
                view += 1;
                break;
            }
        }
        _j += 1
    }
    ans *= view;
    ans
}

pub fn solution_1(filepath: &str) -> i32 {
    let lines: Vec<Vec<i32>> = utils::read_file(filepath)
        .unwrap()
        .iter()
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    let lsums = build_lr_sums(&lines, 0, lines[0].len());
    let rsums = build_lr_sums(&lines, lines[0].len(), 0);
    let dsums = build_ud_sums(&lines, 0, lines.len());
    let usums = build_ud_sums(&lines, lines.len(), 0);

    let rows = lines.len();
    let cols = lines[0].len();
    let mut ans = (rows * 2) + ((cols - 2) * 2);
    for i in 1..(rows - 1) {
        for j in 1..(cols - 1) {
            let cur = lines[i][j]; // At (1, 2): 5
            if cur > lsums[i][j - 1]
                || cur > rsums[i][j + 1]
                || cur > dsums[i - 1][j]
                || cur > usums[i + 1][j]
            {
                ans += 1;
            }
        }
    }
    ans.try_into().unwrap()
}

fn build_lr_sums(lines: &[Vec<i32>], start: usize, end: usize) -> Vec<Vec<i32>> {
    let range: Vec<usize> = if start < end {
        (start..end).into_iter().collect()
    } else {
        (end..start).rev().into_iter().collect()
    };
    lines
        .iter()
        .map(|l| {
            let mut ret = vec![];
            for i in range.clone() {
                if (start < end && i == start) || (start > end && i == start - 1) {
                    ret.push(l[i]);
                    continue;
                }
                let prv = ret.last().unwrap();
                let cur = l[i];
                if prv >= &cur {
                    ret.push(*prv);
                    continue;
                }
                ret.push(cur);
            }
            if start > end {
                ret.reverse();
            }
            ret
        })
        .collect()
}

fn build_ud_sums(lines: &Vec<Vec<i32>>, start: usize, end: usize) -> Vec<Vec<i32>> {
    let range: Vec<usize> = if start < end {
        (start..end).into_iter().collect()
    } else {
        (end..start).rev().into_iter().collect()
    };
    let mut ret = init_vec(lines.len(), lines[0].len());
    let cols = lines[0].len();
    for j in 0..cols {
        let mut v = vec![];
        for i in range.clone() {
            let cur = lines[i][j];
            if (start < end && i == start) || (start > end && i == start - 1) {
                ret[i][j] = cur;
                v.push(cur);
                continue;
            }
            let prv = v.last().unwrap();
            if prv >= &cur {
                let x = *prv;
                v.push(x);
                ret[i][j] = x;
                continue;
            }
            ret[i][j] = cur;
            v.push(cur);
        }
    }
    ret
}

fn init_vec(rows: usize, cols: usize) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    for _ in 0..rows {
        let v = vec![0; cols];
        ret.push(v);
    }
    ret
}
