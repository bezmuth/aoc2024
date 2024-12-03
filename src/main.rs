mod days;
use days::{day01, day02, day03};
use std::fs::File;

fn load_file(path: &str) -> String{
    let mut input = File::open(path).unwrap();
    let mut contents = String::new();
    input.read_to_string(&mut contents).unwrap();
    return contents;
}

fn main() {
    day01::solve(load_file("inputs/input1"));
    day02::solve(load_file("inputs/input2"));
    day03::solve(load_file("inputs/input3"));
}

