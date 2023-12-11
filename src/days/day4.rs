use std::{collections::HashSet, vec};

use crate::utils::read_input;

fn parse_nums(line: &str) -> HashSet<usize> {
    let mut nums: HashSet<usize> = HashSet::new();
    let mut buffer: Vec<String> = Vec::new();

    for (_, char) in line.char_indices() {
        if char.is_whitespace() {
            if let Ok(num) = buffer.concat().parse::<usize>() {
                nums.insert(num);
            }
            buffer = Vec::new();
        } else if char.is_numeric() {
            buffer.push(char.to_string());
        }
    }
    if !buffer.is_empty() {
        if let Ok(num) = buffer.concat().parse::<usize>() {
            nums.insert(num);
        }
    }
    nums
}

pub fn first_part(input: &[String]) -> usize {
    let mut total = 0;

    for line in input.iter() {
        let (_, sub_str) = line.split_once(':').unwrap();

        let (left, right) = sub_str.split_once('|').unwrap();

        let winning_nums = parse_nums(left);
        let card_nums = parse_nums(right);

        let intersection_count = winning_nums.intersection(&card_nums).count();

        let points = if intersection_count == 0 {
            0
        } else {
            2_i32.pow((intersection_count - 1).try_into().unwrap())
        };

        total += points
    }

    total as usize
}

pub fn second_part(input: &Vec<String>) -> usize {
    let mut copied_cards: Vec<usize> = vec![1; input.len()];

    for (idx, line) in input.clone().iter_mut().enumerate() {
        let (_day, sub_str) = line.split_once(':').unwrap();
        let (left, right) = sub_str.split_once('|').unwrap();
        let winning_nums = parse_nums(left);
        let nums = parse_nums(right);

        let intersection_nums = winning_nums.intersection(&nums);
        let intersection_count = intersection_nums.count();

        let range = idx + 1..idx + 1 + intersection_count;
        for j in range {
            copied_cards[j] += copied_cards[idx]
        }
    }
    copied_cards.iter().sum()
}

pub fn execute() {
    let input = &read_input("input/day4.txt");
    let parts = (first_part(input), (second_part(input)));
    println!("Day 4, part 1: {}", parts.0);
    println!("Day 4, part 2: {}", parts.1);
}
