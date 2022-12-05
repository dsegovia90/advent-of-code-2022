use std::fs;

use crate::part_one::calculate_points as calculate_points_1;
use crate::part_two::calculate_points as calculate_points_2;
mod part_one;
mod part_two;


fn main() {
    if let Ok(input) = fs::read_to_string("./input") {
        let result = calculate_points_1(input.clone());
        println!("{}", result);
        let result = calculate_points_2(input);
        println!("{}", result);
    }
}
