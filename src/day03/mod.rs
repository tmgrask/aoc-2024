use regex::{Captures, Regex};
use std::fs;

pub fn part_one() -> i32 {
    let content = fs::read_to_string("src/day03/input.txt").expect("Should read");

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut total = 0;

    for capture in re.captures_iter(&content) {
        let a = &capture[1].parse::<i32>().unwrap();
        let b = &capture[2].parse::<i32>().unwrap();
        total += a * b;
    }

    total
}

#[derive(Debug)]
enum Match {
    Mul { a: i32, b: i32 },
    Do {},
    Dont {},
}

impl From<Captures<'_>> for Match {
    fn from(capture: Captures<'_>) -> Self {
        if let Some(_) = capture.name("mul") {
            Match::Mul {
                a: capture[2].parse::<i32>().unwrap(),
                b: capture[3].parse::<i32>().unwrap(),
            }
        } else if let Some(_) = capture.name("do") {
            Match::Do {}
        } else if let Some(_) = capture.name("dont") {
            Match::Dont {}
        } else {
            panic!("unknown match captured!")
        }
    }
}

pub fn part_two() -> i32 {
    let content = fs::read_to_string("src/day03/input.txt").expect("Should read");

    let re = Regex::new(r"(?P<mul>mul\((\d+),(\d+)\))|(?P<do>do\(\))|(?P<dont>don't\(\))").unwrap();

    let mut total = 0;
    let mut enabled = true;

    for capture in re.captures_iter(&content) {
        let m: Match = capture.into();

        match m {
            Match::Mul { a, b } => {
                if enabled {
                    total += a * b;
                }
            }
            Match::Do {} => {
                if !enabled {
                    enabled = true;
                }
            }
            Match::Dont {} => {
                if enabled {
                    enabled = false;
                }
            }
        }
    }
    total
}
