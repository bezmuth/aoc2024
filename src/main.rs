pub mod days;
use days::{day01, day02, day03};
use std::fs::File;
use std::io::prelude::*;

fn load_file(path: &str) -> String {
    let mut input = File::open(path).unwrap();
    let mut contents = String::new();
    input.read_to_string(&mut contents).unwrap();
    return contents;
}

fn main() {
    println!("Day 1");
    println!("{:?}", day01::solve(load_file("inputs/input1")));
    println!("Day 2");
    println!("{:?}", day02::solve(load_file("inputs/input2")));
    println!("Day 3");
    println!("{:?}", day03::solve(load_file("inputs/input3")));
}
