mod day1;
mod day2;

use std::fs;

fn read_file(name: &str) -> String {
    fs::read_to_string(name).unwrap()
}

fn main() {
    let file = read_file("inputs/day2.sample.txt");
    println!("{}", day2::part_1(&file));
    println!("{}", day2::part_2(&file));
}
