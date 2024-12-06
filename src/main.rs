mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    println!("\nDay 1"); // List sorting
    println!("\tpart 1: {}", day01::part_one());
    println!("\tpart 2: {}", day01::part_two());

    println!("\nDay 2"); // Order parsing with dropout
    println!("\tpart 1: {}", day02::part_one());
    println!("\tpart 2: {}", day02::part_two());

    println!("\nDay 3"); // Regexing multiple captures
    println!("\tpart 1: {}", day03::part_one());
    println!("\tpart 2: {}", day03::part_two());

    println!("\nDay 4"); // 2d word lookup
    println!("\tpart 1: {}", day04::part_one());
    println!("\tpart 2: {}", day04::part_two());

    println!("\nDay 5"); // 2d word lookup
    println!("\tpart 1: {}", day05::part_one());
    println!("\tpart 2: {}", day05::part_two());
}
