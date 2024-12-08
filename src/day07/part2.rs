use std::{collections::VecDeque, fmt};

pub struct TestValue {
    pub total: i64,
    parts: VecDeque<i64>,
}

#[derive(Clone, Debug)]
pub enum Operation {
    Mul,
    Add,
    Concat,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operation::Mul => write!(f, "*"),
            Operation::Add => write!(f, "+"),
            Operation::Concat => write!(f, "||"),
        }
    }
}

impl TestValue {
    pub fn from_line(line: &str) -> Self {
        if let Some((total_raw, parts_raw)) = line.split_once(": ") {
            let total = total_raw.trim().parse().unwrap();
            let mut parts = VecDeque::new();
            for part_raw in parts_raw.trim().split(" ") {
                parts.push_back(part_raw.trim().parse().unwrap());
            }

            TestValue { total, parts }
        } else {
            panic!("Invalid line: {line}");
        }
    }

    fn get_combinatoric_ops(&self) -> Vec<Vec<Operation>> {
        let mut all_ops = Vec::new();
        let length = self.parts.len() - 1;

        if length == 0 {
            return all_ops;
        }

        let total_combinations = 3_usize.pow(length as u32);

        for i in 0..total_combinations {
            let mut current_combination = Vec::with_capacity(length);
            let mut num = i;

            // Convert to base 3 and map each digit to an operation
            for _ in 0..length {
                let operation = match num % 3 {
                    0 => Operation::Mul,
                    1 => Operation::Add,
                    2 => Operation::Concat,
                    _ => unreachable!(),
                };
                current_combination.push(operation);
                num /= 3;
            }

            all_ops.push(current_combination);
        }

        all_ops
    }

    fn ops_work(&self, ops: &Vec<Operation>) -> bool {
        let mut parts = self.parts.clone();

        let mut total = parts.pop_front().unwrap();
        for op in ops {
            if total > self.total {
                return false;
            }
            let part = parts.pop_front().unwrap();
            match op {
                Operation::Mul => total *= part,
                Operation::Add => total += part,
                Operation::Concat => {
                    let concatenated = format!("{}{}", total, part)
                        .parse::<i64>()
                        .expect("Failed to parse concatenated number");
                    total = concatenated;
                }
            };
        }

        if total == self.total {
            return true;
        }

        false
    }

    pub fn find_solutions(&self) -> Vec<Vec<Operation>> {
        let mut working_ops = Vec::new();
        for ops in self.get_combinatoric_ops() {
            //println!("{:?}", ops);
            if self.ops_work(&ops) {
                working_ops.push(ops);
            }
        }

        working_ops
    }

    #[allow(dead_code)]
    pub fn format(&self, parts: &mut VecDeque<i64>, ops: Vec<Operation>) -> String {
        let mut out = format!("{}", parts.pop_front().unwrap());

        for (op, part) in ops.iter().zip(parts.iter()) {
            out += &format!(" {} {}", op, part);
        }

        out
    }
}
