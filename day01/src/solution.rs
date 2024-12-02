use std::str::FromStr;
use util::read_input;

pub fn solution(file_name: &str) -> String {
    let input = read_input(&format!("./day01/{}", file_name)).unwrap();
    let input = input.trim();
    let data = ParsedData::from_str(&input).unwrap();
    data.total_distance().to_string()
}

struct ParsedData {
    left_list: Vec<u32>,
    right_list: Vec<u32>,
}

impl ParsedData {
    pub fn total_distance(&self) -> u32 {
        self.left_list.iter().zip(self.right_list.iter()).map(|(&l, &r)| r.abs_diff(l)).sum()
    }
}

impl FromStr for ParsedData {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (mut left_list, mut right_list): (Vec<u32>, Vec<u32>) = input
            .lines()
            .map(|line| {
                let mut iter = line.split_whitespace();
                let left: u32 = iter.next().unwrap().parse().unwrap(); //TODO proper error handling?
                let right: u32 = iter.next().unwrap().parse().unwrap();
                (left, right)
            })
            .collect();
        left_list.sort();
        right_list.sort();

        Ok(ParsedData {
            left_list,
            right_list,
        })
    }
}
