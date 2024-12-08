use rayon::prelude::*;
use std::fs;

mod part1;
mod part2;

pub fn part_one(file: &str) -> i64 {
    let content = fs::read_to_string(file).expect("Should read");

    let mut calibration_result = 0;
    for line in content.lines() {
        let test_value = part1::TestValue::from_line(line);
        let solutions = test_value.find_solutions();

        if solutions.len() > 0 {
            calibration_result += test_value.total;
        }
    }

    calibration_result
}

pub fn part_two(file: &str) -> i64 {
    let content = fs::read_to_string(file).expect("Should read");

    content
        .lines()
        .par_bridge()
        .map(|line| {
            let test_value = part2::TestValue::from_line(line);
            let solutions = test_value.find_solutions();
            if solutions.is_empty() {
                0
            } else {
                test_value.total
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let result = part_one(&"src/day07/test.txt");
        assert_eq!(result, 3749);
    }

    #[test]
    fn p2() {
        let result = part_two(&"src/day07/test.txt");
        assert_eq!(result, 11387);
    }
}
