mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

#[macro_export]
macro_rules! time_it {
    ($e:expr) => {{
        let start = std::time::Instant::now();
        let result = $e;
        let duration = start.elapsed();
        println!(
            "\t{} in {:>6.1?}ms -> {:?}",
            stringify!($e),
            duration.as_secs_f64() * 1000.0,
            result,
        );
        result
    }};
}

fn main() {
    println!("\n\tAdvent of Code 2024\n");

    // Basic list sorting
    time_it!(day01::part_one("src/day01/input.txt"));
    time_it!(day01::part_two("src/day01/input.txt"));
    println!("");

    // Order parsing with dropout
    time_it!(day02::part_one("src/day02/input.txt"));
    time_it!(day02::part_two("src/day02/input.txt"));
    println!("");

    // Regexing multiple captures
    time_it!(day03::part_one("src/day03/input.txt"));
    time_it!(day03::part_two("src/day03/input.txt"));
    println!("");

    // 2d word lookup
    time_it!(day04::part_one("src/day04/input.txt"));
    time_it!(day04::part_two("src/day04/input.txt"));
    println!("");

    // Unusual ordering and sort
    time_it!(day05::part_one("src/day05/input.txt"));
    time_it!(day05::part_two("src/day05/input.txt"));
    println!("");

    // Path finding
    time_it!(day06::part_one("src/day06/input.txt"));
    time_it!(day06::part_two("src/day06/input.txt"));
    println!("");

    // Kenken
    time_it!(day07::part_one("src/day07/input.txt"));
    time_it!(day07::part_two("src/day07/input.txt"));
    println!("");

    // Resonant collinearity
    time_it!(day08::part_one("src/day08/input.txt"));
    time_it!(day08::part_two("src/day08/input.txt"));
    println!("");

    // Defragmentation
    time_it!(day09::part_one("src/day09/input.txt"));
    time_it!(day09::part_two("src/day09/input.txt"));
    println!("");

    // Topology search
    time_it!(day10::part_one("src/day10/input.txt"));
    time_it!(day10::part_two("src/day10/input.txt"));
    println!("");

    // Stones with numbers on 'em
    time_it!(day11::part_one("src/day11/input.txt"));
    time_it!(day11::part_two("src/day11/input.txt"));
    println!("");
}
