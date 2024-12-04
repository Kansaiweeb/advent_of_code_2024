pub(crate) fn solve() {
    let day01 = day01::solution::solution("input.txt");
    println!("day01 a: {}, b: {}", day01.0, day01.1);
    let day02 = day02::solution::solution("input.txt");
    println!("day01 a: {}, b: {}", day02.0, day02.1);
}

pub(crate) fn solve_examples() {
    let day01 = day01::solution::solution("example_input.txt");
    println!("day01 a: {}, b: {}", day01.0, day01.1);
    let day02 = day02::solution::solution("example_input.txt");
    println!("day01 a: {}, b: {}", day02.0, day02.1);
}
