use num::{rational::Ratio, BigInt, BigRational, FromPrimitive, ToPrimitive};
use regex::Regex;
use std::fs;

struct Coord {
    x: Ratio<BigInt>,
    y: Ratio<BigInt>,
}

struct Machine {
    a: Coord,
    b: Coord,
    prize: Coord,
}

impl Machine {
    // rust is clunky for math?
    fn solution(&self) -> Option<i64> {
        // A*a.x + B*b.x = prize.x
        // A*a.y + B*b.y = prize.y
        //
        // A = (prize.x - B*b.x) / a.x
        // A = (prize.y - B*b.y) / a.y
        // prize.x/a.x - B*b.x/a.x = prize.y/a.y - B*b.y/a.y
        // B*b.x/a.x - B*b.y/a.y = prize.x/a.x - prize.y/a.y
        //
        // so, solve these two:
        // B = (prize.x/a.x - prize.y/a.y)/(b.x/a.x - b.y/a.y)
        // A = (prize.x - B*b.x) / a.x

        let b_presses = ((&self.prize.x / &self.a.x) - (&self.prize.y / &self.a.y))
            / ((&self.b.x / &self.a.x) - (&self.b.y / &self.a.y));
        let a_presses = (&self.prize.x - (&b_presses * &self.b.x)) / &self.a.x;

        if !b_presses.is_integer() || !a_presses.is_integer() {
            return None;
        }

        let a_tokens = a_presses.to_i64().unwrap() * 3;
        let b_tokens = b_presses.to_i64().unwrap() * 1;

        Some(a_tokens + b_tokens)
    }
}

fn get_machines(content: &str, prize_mod: i64) -> Vec<Machine> {
    let pattern = r"(?:Button A: X\+(?P<ax>\d+), Y\+(?P<ay>\d+)\nButton B: X\+(?P<bx>\d+), Y\+(?P<by>\d+)\nPrize: X=(?P<px>\d+), Y=(?P<py>\d+))";
    let re = Regex::new(pattern).unwrap();

    let mut machines = Vec::new();
    for caps in re.captures_iter(content) {
        machines.push(Machine {
            a: Coord {
                x: BigRational::from_i64(caps["ax"].parse::<i64>().unwrap()).unwrap(),
                y: BigRational::from_i64(caps["ay"].parse::<i64>().unwrap()).unwrap(),
            },
            b: Coord {
                x: BigRational::from_i64(caps["bx"].parse::<i64>().unwrap()).unwrap(),
                y: BigRational::from_i64(caps["by"].parse::<i64>().unwrap()).unwrap(),
            },
            prize: Coord {
                x: BigRational::from_i64(caps["px"].parse::<i64>().unwrap() + prize_mod).unwrap(),
                y: BigRational::from_i64(caps["py"].parse::<i64>().unwrap() + prize_mod).unwrap(),
            },
        });
    }

    machines
}

pub fn part_one(file: &str) -> i64 {
    let content = fs::read_to_string(file).expect("File should exist and be readable");

    let machines = get_machines(&content, 0);

    let mut total = 0;
    for machine in machines {
        match machine.solution() {
            Some(tokens) => total += tokens,
            None => {}
        }
    }

    total
}

pub fn part_two(file: &str) -> i64 {
    let content = fs::read_to_string(file).expect("File should exist and be readable");

    let machines = get_machines(&content, 10000000000000);

    let mut total = 0;
    for machine in machines {
        match machine.solution() {
            Some(tokens) => total += tokens,
            None => {}
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let result = part_one(&"src/day13/test.txt");
        assert_eq!(result, 480);
    }

    #[test]
    fn p2() {
        let result = part_two(&"src/day13/test.txt");
        assert_eq!(result, 875318608908);
    }
}
