mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

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
    time_it!(day01::part_one());
    time_it!(day01::part_two());
    println!("");

    // Order parsing with dropout
    time_it!(day02::part_one());
    time_it!(day02::part_two());
    println!("");

    // Regexing multiple captures
    time_it!(day03::part_one());
    time_it!(day03::part_two());
    println!("");

    // 2d word lookup
    time_it!(day04::part_one());
    time_it!(day04::part_two());
    println!("");

    // Unusual ordering and sort
    time_it!(day05::part_one());
    time_it!(day05::part_two());
    println!("");

    // Path finding
    time_it!(day06::part_one());
    time_it!(day06::part_two());
    println!("");

    // Kenken
    time_it!(day07::part_one());
    time_it!(day07::part_two());
    println!("");
}
