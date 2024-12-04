use std::iter::zip;
fn left_parse(contents: &str) -> Vec<isize> {
    let mut out = contents
        .lines()
        // split the two columns
        .map(|line| line.split("   "))
        // get the left column
        .map(|mut line| line.next().unwrap().parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    out.sort_unstable();
    out
}
fn right_parse(contents: &str) -> Vec<isize> {
    let mut out = contents
        .lines()
        .map(|line| line.split("   "))
        .map(|mut line| line.nth(1).unwrap().parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    out.sort_unstable();
    out
}
fn part1(contents: &str) -> isize {
    let left: Vec<isize> = left_parse(contents);
    let right: Vec<isize> = right_parse(contents);

    let distance: isize = zip(left, right)
        // get the distance between the two columns
        .map(|nums| {
            if nums.0 > nums.1 {
                nums.0 - nums.1
            } else {
                nums.1 - nums.0
            }
        })
        .sum();
    distance as isize
}
fn part2(contents: &str) -> isize {
    let left: Vec<isize> = left_parse(contents);
    let right: Vec<isize> = right_parse(contents);

    let total: isize = left
        .iter()
        .map(|x| x * isize::try_from(right.iter().filter(|y| *y == x).count()).unwrap())
        .sum();

    total as isize
}
pub fn solve(input: &str) -> (isize, isize) {
    (part1(input), part2(input))
}
