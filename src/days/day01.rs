use std::iter::zip;

fn part1(contents: String) {
    let mut left: Vec<i32> = contents
        .lines()
        // split the two columns
        .map(|line| line.split("   ").collect::<Vec<&str>>())
        // get the left column
        .map(|line| line[0].parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    left.sort();

    let mut right: Vec<i32> = contents
        .lines()
        .map(|line| line.split("   ").collect::<Vec<&str>>())
        .map(|line| line[1].parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    right.sort();

    let distance: i32 = zip(left, right)
        // get the distance between the two columns
        .map(|nums| if nums.0 > nums.1 { nums.0-nums.1} else {nums.1-nums.0})
        .sum();
    println!("Part 1: {:?}", distance);
}

fn part2(contents: String) {
    let mut left: Vec<i32> = contents
        .lines()
        .map(|line| line.split("   ").collect::<Vec<&str>>())
        .map(|line| line[0].parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    left.sort();

    let mut right: Vec<i32> = contents
        .lines()
        .map(|line| line.split("   ").collect::<Vec<&str>>())
        .map(|line| line[1].parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    right.sort();

    let total: i32 = left.iter().map(|x| x*(right.iter().filter(|y| *y == x).count() as i32)).sum();

    println!("Part 2: {:?}", total);
}

pub fn solve(input: String) {
    part1(input.clone());
    part2(input.clone())
}
