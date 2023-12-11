use std::collections::HashMap;

use crate::utils::read_input;

pub fn first_part(input: &[String]) -> usize {
    let mut count = 0;
    input.iter().for_each(|line| {
        let mut nums: Vec<usize> = Vec::new();
        line.split("").for_each(|x| {
            if let Ok(x) = x.parse::<usize>() {
                nums.push(x);
            }
        });
        count += nums.first().unwrap() * 10 + nums.last().unwrap();
    });
    count
}

pub fn second_part(input: &[String]) -> usize {
    let mappings: HashMap<&str, usize> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .cloned()
    .collect();

    let mut count = 0;

    for line in input.iter() {
        let mut remaining_line = line.as_str();
        let mut nums: Vec<usize> = Vec::new();

        'outer: while !remaining_line.is_empty() {
            for (word, num) in mappings.iter() {
                if remaining_line.starts_with(word) {
                    nums.push(*num);
                    remaining_line = &remaining_line[word.len() - 1..];
                    continue 'outer;
                }
            }

            let line_bytes = remaining_line.as_bytes();
            if line_bytes[0].is_ascii_digit() {
                let num = line_bytes[0] as char;
                nums.push(num.to_digit(10).unwrap() as usize);
            }
            remaining_line = &remaining_line[1..];
        }
        let combined = nums.first().unwrap() * 10 + nums.last().unwrap();
        count += combined;
    }
    count
}

pub fn execute() {
    let input = &read_input("input/day1.txt");
    let parts = (first_part(input), (second_part(input)));
    println!("Day 1, part 1: {}", parts.0);
    println!("Day 1, part 2: {}", parts.1);
}
