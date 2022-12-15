use crate::yr_2022::problems::utils;

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
    println!("{:?}", sensors);
    println!("{:?}", beacons);
    0
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
