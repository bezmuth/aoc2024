use std::iter::zip;
fn left_parse(contents: &str) -> Vec<i32> {
    let mut out = contents
        .lines()
        // split the two columns
        .map(|line| line.split("   "))
        // get the left column
        .map(|mut line| line.next().unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    out.sort();
    return out;
}
fn right_parse(contents: &str) -> Vec<i32> {
    let mut out = contents
        .lines()
        .map(|line| line.split("   "))
        .map(|mut line| line.nth(1).unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    out.sort();
    return out;
}
fn part1(contents: &str) -> isize {
    let left: Vec<i32> = left_parse(&contents);
    let right: Vec<i32> = right_parse(&contents);

    let distance: i32 = zip(left, right)
        // get the distance between the two columns
        .map(|nums| {
            if nums.0 > nums.1 {
                nums.0 - nums.1
            } else {
                nums.1 - nums.0
            }
        })
        .sum();
    return distance as isize;
}
fn part2(contents: &str) -> isize {
    let left: Vec<i32> = left_parse(&contents);
    let right: Vec<i32> = right_parse(&contents);

    let total: i32 = left
        .iter()
        .map(|x| x * (right.iter().filter(|y| *y == x).count() as i32))
        .sum();

    return total as isize;
}

pub fn solve(input: String) -> (isize, isize) {
    (part1(&input), part2(&input))
}
