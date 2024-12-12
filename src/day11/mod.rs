use std::{collections::HashMap, fs};

fn get_stones(content: &str) -> Vec<usize> {
    let mut stones = Vec::new();

    for stone_raw in content.trim().split(" ") {
        stones.push(stone_raw.parse::<usize>().unwrap());
    }

    stones
}

fn apply_rules(stone: &usize) -> Vec<usize> {
    let stone_str = stone.to_string();
    if *stone == 0 {
        vec![1]
    } else if stone_str.len() % 2 == 0 {
        let left = stone_str[..stone_str.len() / 2].parse::<usize>().unwrap();
        let right = stone_str[(stone_str.len() / 2)..].parse::<usize>().unwrap();
        vec![left, right]
    } else {
        vec![stone * 2024]
    }
}

pub fn part_one(file: &str) -> usize {
    let content = fs::read_to_string(file).expect("File should exist and be readable to string");

    let mut stones = get_stones(&content);
    let mut new_stones = Vec::new();

    for _ in 0..25 {
        for stone in &stones {
            new_stones.extend(apply_rules(stone));
        }
        stones = new_stones;
        new_stones = Vec::new();
    }

    stones.len()
}

pub fn part_two(file: &str) -> usize {
    let content = fs::read_to_string(file).expect("File should exist and be readable to string");

    let mut stones = get_stones(&content)
        .into_iter()
        .map(|value| (value, 1))
        .collect::<HashMap<usize, usize>>();

    for _ in 0..75 {
        let mut new_stones = HashMap::new();
        for (stone, old_count) in stones {
            for new_stone in apply_rules(&stone) {
                let new_count = new_stones.entry(new_stone).or_default();
                *new_count += old_count;
            }
        }
        stones = new_stones;
    }

    stones.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(apply_rules(&0), [1]);
        assert_eq!(apply_rules(&1250), [12, 50]);
        assert_eq!(apply_rules(&1000), [10, 0]);
        assert_eq!(apply_rules(&125), [253000]);
        let result = part_one(&"src/day11/test.txt");
        assert_eq!(result, 55312);
    }
}
