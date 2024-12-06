use std::fs;

pub fn part_one() -> i32 {
    let contents =
        fs::read_to_string("src/day01/input.txt").expect("Should have been able to read input.txt");

    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in contents.lines() {
        let mut elems = line.split_whitespace();
        left.push(elems.next().unwrap().parse::<i32>().unwrap());
        right.push(elems.next().unwrap().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let pairs = left.iter().zip(right.iter());

    let mut differences = 0;

    for pair in pairs {
        let difference = pair.0 - pair.1;
        differences += difference.abs();
        //println!("{:?} difference {difference}, differences: {differences}", pair);
    }

    differences
}

pub fn part_two() -> i32 {
    let contents =
        fs::read_to_string("src/day01/input.txt").expect("Should have been able to read input.txt");

    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in contents.lines() {
        let mut elems = line.split_whitespace();
        left.push(elems.next().unwrap().parse::<i32>().unwrap());
        right.push(elems.next().unwrap().parse::<i32>().unwrap());
    }

    let mut similarity = 0;

    for value in left.iter() {
        let right_indices: Vec<usize> = right
            .iter()
            .enumerate()
            .filter(|(_, val)| *val == value)
            .map(|(i, _)| i)
            .collect();
        similarity += value * (right_indices.len() as i32);
    }

    similarity
}
