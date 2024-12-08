use std::{cmp::Ordering, fs};

fn prepare_data(content: &str) -> (Vec<Vec<&str>>, Vec<&str>) {
    let Some((ordering_raw, updates_raw)) = content.split_once("\n\n") else {
        panic!("bad split")
    };

    let mut ordering = Vec::new();
    for pair in ordering_raw.split("\n") {
        ordering.push(pair);
    }

    let mut updates = Vec::new();
    for update_raw in updates_raw.split("\n") {
        if update_raw == "" {
            continue;
        }

        let mut numbers = Vec::new();
        for number in update_raw.split(",") {
            numbers.push(number);
        }

        updates.push(numbers);
    }

    (updates, ordering)
}

fn pair_is_valid(first: &str, last: &str, ordering: &Vec<&str>) -> bool {
    let invalid_pair = format!("{}|{}", last, first);
    if ordering.contains(&invalid_pair.as_str()) {
        //println!("invalid pair found {:?}", invalid_pair);
        return false;
    }
    true
}

fn order_is_valid(mut numbers: Vec<&str>, ordering: &Vec<&str>) -> bool {
    numbers.reverse();
    while let Some(this_number) = numbers.pop() {
        for later_number in &mut *numbers {
            if !pair_is_valid(this_number, later_number, ordering) {
                return false;
            }
        }
    }
    true
}

fn get_middle_sum(updates: Vec<Vec<&str>>) -> i32 {
    let mut middle_sum = 0;
    for update in updates {
        let middle_raw = update[update.len() / 2];
        middle_sum += middle_raw.parse::<i32>().unwrap();
    }
    middle_sum
}

pub fn part_one(file: &str) -> i32 {
    let content = fs::read_to_string(file).expect("Should read");

    let (updates, ordering) = prepare_data(content.as_str());

    let mut correct_updates = Vec::new();
    for update in updates {
        if order_is_valid(update.clone(), &ordering) {
            correct_updates.push(update);
        }
    }

    get_middle_sum(correct_updates)
}

pub fn part_two(file: &str) -> i32 {
    let content = fs::read_to_string(file).expect("Should read");

    let (updates, ordering) = prepare_data(content.as_str());

    let mut incorrect_updates = Vec::new();
    for update in updates {
        if !order_is_valid(update.clone(), &ordering) {
            incorrect_updates.push(update);
        }
    }

    let mut sorted_updates = Vec::new();
    for mut update in incorrect_updates.clone() {
        update.sort_by(|first, last| {
            if pair_is_valid(first, last, &ordering) {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        });
        sorted_updates.push(update);
    }

    get_middle_sum(sorted_updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let result = part_one(&"src/day05/test.txt");
        assert_eq!(result, 143);
    }

    #[test]
    fn p2() {
        let result = part_two(&"src/day05/test.txt");
        assert_eq!(result, 123);
    }
}
