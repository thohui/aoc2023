use std::collections::HashSet;
use std::fmt;

use crate::utils::read_input;

#[derive(Debug, Clone)]
struct Number {
    coords: HashSet<(i64, i64)>,
    value: String,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl Number {
    fn new() -> Self {
        Self {
            coords: HashSet::new(),
            value: String::new(),
        }
    }
    fn append_char(&mut self, ch: char) {
        self.value.push(ch);
    }
    fn get_value(&self) -> Option<usize> {
        if let Ok(num) = self.value.parse::<usize>() {
            return Some(num);
        }
        None
    }
}

fn first_part(input: Vec<String>) -> usize {
    let mut symbol_indices: HashSet<(i64, i64)> = HashSet::new();
    let lines: Vec<Vec<char>> = input
        .iter()
        .map(|line| {
            let mut chars = line.chars().to_owned().collect::<Vec<char>>();
            chars.push('.');
            chars
        })
        .collect();

    lines.iter().enumerate().for_each(|(x, row)| {
        for (y, value) in row.iter().enumerate() {
            if !(*value).is_numeric() && *value != '.' {
                symbol_indices.insert((x as i64, y as i64));
            }
        }
    });

    println!("{:?}", symbol_indices);

    let mut nums: Vec<Number> = vec![];

    for (row, line) in lines.iter().enumerate() {
        let mut current_number: Option<Number> = None;
        for (col, char) in line.iter().enumerate() {
            if char.is_numeric() {
                let coords = [
                    ((row as i64) + 1, col as i64),
                    ((row as i64) - 1, col as i64),
                    (row as i64, (col as i64) + 1),
                    (row as i64, (col as i64) - 1),
                    ((row as i64) + 1, (col as i64) + 1),
                    ((row as i64) - 1, (col as i64) + 1),
                    ((row as i64) + 1, (col as i64) - 1),
                    ((row as i64) - 1, (col as i64) - 1),
                ];

                if let Some(ref mut number) = current_number {
                    number.coords.extend(coords.iter().cloned());
                    number.append_char(*char);
                } else {
                    let mut new_number = Number::new();
                    new_number.coords.extend(coords.iter().cloned());
                    new_number.append_char(*char);
                    current_number = Some(new_number);
                }
            } else if let Some(ref number) = current_number {
                if number.get_value().is_some() {
                    nums.push(number.clone());
                }
                current_number = None;
            }
        }
    }

    nums.into_iter()
        .filter(|num| num.coords.intersection(&symbol_indices).count() > 0)
        .map(|num| num.get_value().unwrap())
        .sum()
}

fn second_part(input: Vec<String>) -> usize {
    let mut gears: HashSet<(i64, i64)> = HashSet::new();
    let lines: Vec<Vec<char>> = input
        .iter()
        .map(|line| {
            let mut chars = line.chars().to_owned().collect::<Vec<char>>();
            chars.push('.');
            chars
        })
        .collect();

    lines.iter().enumerate().for_each(|(row, line)| {
        for (col, value) in line.iter().enumerate() {
            if *value == '*' {
                gears.insert((row as i64, col as i64));
            }
        }
    });

    let mut nums: Vec<Number> = vec![];

    for (row, line) in lines.iter().enumerate() {
        let mut current_number: Option<Number> = None;
        for (col, char) in line.iter().enumerate() {
            if char.is_numeric() {
                let coords = [
                    ((row as i64) + 1, col as i64),
                    ((row as i64) - 1, col as i64),
                    (row as i64, (col as i64) + 1),
                    (row as i64, (col as i64) - 1),
                    ((row as i64) + 1, (col as i64) + 1),
                    ((row as i64) - 1, (col as i64) + 1),
                    ((row as i64) + 1, (col as i64) - 1),
                    ((row as i64) - 1, (col as i64) - 1),
                ];

                if let Some(ref mut number) = current_number {
                    number.coords.extend(coords.iter().cloned());
                    number.append_char(*char);
                } else {
                    let mut new_number = Number::new();
                    new_number.coords.extend(coords.iter().cloned());
                    new_number.append_char(*char);
                    current_number = Some(new_number);
                }
            } else if let Some(ref number) = current_number {
                if number.get_value().is_some() {
                    nums.push(number.clone());
                }
                current_number = None;
            }
        }
    }
    let mut sum = 0;
    for gear in gears {
        let part_numbers = nums
            .iter()
            .filter(|num| num.coords.contains(&gear))
            .collect::<Vec<&Number>>();
        if part_numbers.len() == 2 {
            sum += part_numbers[0].get_value().unwrap() * part_numbers[1].get_value().unwrap();
        }
    }
    sum
}

pub fn execute() {
    let input = &read_input("input/day3.txt");
    let parts = (first_part(input.clone()), (second_part(input.clone())));
    println!("Day 3, part 1: {}", parts.0);
    println!("Day 3, part 2: {}", parts.1);
}
