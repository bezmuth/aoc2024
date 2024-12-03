use std::fs::File;
use std::io::prelude::*;

fn load_input() -> String {
    let mut input = File::open("inputs/input3").unwrap();
    let mut contents = String::new();
    input.read_to_string(&mut contents).unwrap();
    return contents;
}

fn parse_input1(input: String) -> isize {
    let parsed = input
        .split("mul(")
        .map(|instruction| {
            let mut nums = (0, 0);
            let mut state = 0; // 0 - num 1, 1 - num 2, 2 - right bracket
            for ch in instruction.chars() {
                if ch.is_numeric() && state == 0 {
                    nums.0 = (nums.0 * 10) + (ch.to_string()).parse::<isize>().unwrap();
                } else if ch == ',' && state == 0 {
                    state = 1;
                } else if ch.is_numeric() && state == 1 {
                    nums.1 = (nums.1 * 10) + (ch.to_string()).parse::<isize>().unwrap();
                } else if ch == ')' && state == 1 {
                    state = 2
                } else if state != 2 {
                    return (0, 0);
                }
            }
            return nums;
        })
        .collect::<Vec<(isize, isize)>>();

    parsed.iter().map(|(a, b)| a * b).sum()
}

fn parse_input2(input: String) -> isize {
    let preprocess = input
        .split("do()") // split along the dos
        .map(|x| x.split("don't()").collect::<Vec<_>>()[0]) // now remove anything after dont
        .collect::<Vec<&str>>()
        .join("");

    return parse_input1(preprocess);
}

pub fn solve() {
    println!("{:?}", parse_input1(load_input()));
    println!("{:?}", parse_input2(load_input()));
}
