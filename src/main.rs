mod one;
mod two;
mod three;
mod four;

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

    println!("\nDay 4"); // 2d word lookup
    println!("\tpart 1: {}", four::part_one());
    println!("\tpart 2: {}", four::part_two());
}
