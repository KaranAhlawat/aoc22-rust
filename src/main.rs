mod day1;
mod day2;
mod day3;

use std::fs;

fn read_file(name: &str) -> String {
    fs::read_to_string(name).unwrap()
}

fn main() {
    let file = read_file("inputs/day3.input.txt");
    println!("{}", day3::part_1(&file));
    println!("{}", day3::part_2(&file));
}
