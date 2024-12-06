mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

#[macro_export]
macro_rules! time_it {
    ($e:expr) => {{
        let start = std::time::Instant::now();
        let result = $e;
        let duration = start.elapsed();
        println!(
            "\t{} in {:?}ms\t -> {:?}",
            stringify!($e),
            duration.as_millis(),
            result,
        );
        result
    }};
}

fn main() {
    // Basic list sorting
    time_it!(day01::part_one());
    time_it!(day01::part_two());

    // Order parsing with dropout
    time_it!(day02::part_one());
    time_it!(day02::part_two());

    // Regexing multiple captures
    time_it!(day03::part_one());
    time_it!(day03::part_two());

    // 2d word lookup
    time_it!(day04::part_one());
    time_it!(day04::part_two());

    // Unusual ordering and sort
    time_it!(day05::part_one());
    time_it!(day05::part_two());
}
