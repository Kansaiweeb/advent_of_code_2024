use std::cmp::PartialEq;
use std::str::FromStr;
use util::read_input;

pub fn solution(file_name: &str) -> (String, String) {
    let input = read_input(&format!("./day02/{}", file_name)).unwrap();
    let input = input.trim();
    let amount_of_safe_reports = input.lines().filter(|line| is_safe(line)).count();

    (
        amount_of_safe_reports.to_string(),
        amount_of_safe_reports.to_string(),
    )
}

fn is_safe(report: &str) -> bool {
    println!("report {:?}", report);
    let report_vec: Vec<i32> = report
        .split_whitespace()
        .map(|element| i32::from_str(element).unwrap())
        .collect();
    let mut windows = report_vec.windows(2);
    let mut sort_direction = None;
    while let Some(window) = windows.next() {
        if window.len() != 2 {
            continue;
        }
        let [left, right] = window else { continue };
        let diff = left - right;
        let current_direction = if diff > 0 {
            Some(SortDirection::ASC)
        } else {
            Some(SortDirection::DESC)
        };
        match &sort_direction {
            None => sort_direction = current_direction,
            Some(sort_direction) => {
                if *sort_direction != current_direction.unwrap() {
                    println!("inconsistent sort direction");
                    return false;
                }
            }
        }
        if !(1..=3).contains(&diff.abs()) {
            println!("not in range 1..3");
            return false;
        };
    }
    println!("passed");
    true
}

#[derive(PartialEq)]
enum SortDirection {
    DESC,
    ASC,
}

impl Default for SortDirection {
    fn default() -> Self {
        Self::ASC
    }
}
