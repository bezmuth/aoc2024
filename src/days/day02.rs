use itertools::Itertools;
fn load_input(contents: String) -> Vec<Vec<isize>> {
    // converts the string into the correct format (array of ints by line)
    return contents
        .lines()
        .map(|line| line.split(" "))
        .map(|line| line.map(|num| num.parse::<isize>().unwrap()).collect())
        .collect::<Vec<Vec<isize>>>();
}
fn good_reactor_fold(acc: isize, x: (&isize, &isize)) -> Result<isize, ()> {
    // check if decreased prevously, check if currently currently decreasing and if input valid
    if acc <= 0 && x.0 - x.1 <= 3 && x.0 - x.1 >= 1 {
        // decreasing
        Ok(-1)
    } else if acc >= 0 && x.1 - x.0 <= 3 && x.1 - x.0 >= 1 {
        // increasing
        Ok(1)
    } else {
        Err(())
    }
}
fn part1(input: String) -> usize {
    load_input(input)
        .iter()
        .filter(|row| {
            row.into_iter()
                // gives current and next val as tuple
                .tuple_windows()
                .try_fold(0, good_reactor_fold)
                .is_ok()
        })
        .count()
}
fn part2(input: String) -> usize {
    load_input(input)
        .iter()
        .filter(|row| {
            row.iter()
                // all possible combinations of the nums if one was removed
                .combinations(row.len() - 1)
                .map(|row_perm| {
                    row_perm
                        .into_iter()
                        .tuple_windows()
                        .try_fold(0, good_reactor_fold)
                        .is_ok()
                })
                .any(|x| x)
        })
        .count()
}
pub fn solve(input: String) -> (usize, usize) {
    (part1(input.clone()), part2(input.clone()))
}
