use std::fs::File;
use std::io::{self, BufRead};

pub fn read_file(filename: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut lines_as_vec = vec![];
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    for line in lines.flatten() {
        lines_as_vec.push(line)
    }
    Ok(lines_as_vec)
}

#[allow(dead_code)]
pub fn convert_to_vec_of_ints(board: Vec<String>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    for (_, line) in board.iter().enumerate() {
        let mut row = vec![];
        for (_, char) in line.chars().into_iter().enumerate() {
            row.push(char.to_digit(10).unwrap().try_into().unwrap())
        }
        ans.push(row)
    }
    ans
}
