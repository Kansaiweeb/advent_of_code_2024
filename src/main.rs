extern crate day01;
extern crate day02;

use std::env;

mod solutions;

fn main() {
    let example: bool = {
        let args: Vec<String> = env::args().collect();
        println!("{:?}", args);

        match args.get(1) {
            None => false,
            Some(arg) => arg == "example",
        }
    };

    if example {
        solutions::solve_examples();
    } else {
        solutions::solve()
    }
}
