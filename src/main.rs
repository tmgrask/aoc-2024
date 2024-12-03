mod one;
mod two;
mod three;

fn main() {
    println!("\nDay 1"); // List sorting
    println!("\tpart 1: {}", one::part_one());
    println!("\tpart 2: {}", one::part_two());

    println!("\nDay 2"); // Order parsing with dropout
    println!("\tpart 1: {}", two::part_one());
    println!("\tpart 2: {}", two::part_two());

    println!("\nDay 3"); // Regexing multiple captures
    println!("\tpart 1: {}", three::part_one());
    println!("\tpart 2: {}", three::part_two());
}
