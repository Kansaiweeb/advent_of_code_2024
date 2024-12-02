
pub fn solve_all() {
}

pub(crate) fn solve() {
    let day01 = day01::solution::solution("input.txt");
    println!("day01 a: {}, b: {}", day01.0, day01.1);
}

pub(crate) fn solve_examples() {
    let day01 = day01::solution::solution("example_input.txt");
    println!("day01 a: {}, b: {}", day01.0, day01.1);
}