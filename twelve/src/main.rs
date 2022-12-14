use std::fs;

mod part_one;
mod part_two;


fn main() {
    if let Ok(input) = fs::read_to_string("./input") {
        let result_1 = part_one::calculate(input.clone());
        println!("{}", result_1);
        let result_2 = part_two::calculate(input);
        println!("{}", result_2);
    }
}
