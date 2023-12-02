use std::collections::HashMap;

use crate::utils::read_input;

fn calculate_vec(vec: &mut Vec<String>) -> usize {
    println!("calculating {:?}", vec);
    let mut count = 0;
    let mut first = vec.first_mut().cloned();
    let last = vec.last();
    if let Some(first) = first.as_mut() {
        if let Some(last) = last {
            first.push_str(last);
        }
        count += first.parse::<usize>().unwrap();
    }
    return count;
}

pub fn first_part(input: &Vec<String>) -> usize {
    let mut count = 0;
    input.iter().for_each(|line| {
        let mut num_str = Vec::<String>::new();
        line.split("").for_each(|x| {
            if x.parse::<usize>().is_ok() {
                num_str.push(x.into());
            }
        });
        count += calculate_vec(&mut num_str);
    });
    count
}

pub fn second_part(input: &Vec<String>) -> usize {
    let mut mappings: HashMap<&str, &str> = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]
    .iter()
    .cloned()
    .collect();
    let mut count = 0;
    input.iter().for_each(|line| {
        let mut num_vec = Vec::<String>::new();
        let mut num_str = String::new();
        line.split("").for_each(|x| {
            num_str += x;
            if x.parse::<usize>().is_ok() {
                println!("value: {}", num_str);
                num_vec.push(x.to_string());
                num_str = String::new();
            } else {
                if let Some(value) = mappings.get_mut(num_str.as_str()) {
                    println!("value: {}", num_str);
                    num_vec.push(value.to_string());
                    num_str = String::new();
                }
            }
        });
        count += calculate_vec(&mut num_vec);
    });
    count
}

pub fn execute() {
    let input = &read_input("input/example.txt");
    let parts = (first_part(input), (second_part(input)));
    println!("Day 1, part 1: {}", parts.0);
    println!("Day 1, part 2: {}", parts.1);
}
