use crate::utils::read_input;

pub fn first_part(input: &[String]) -> usize {
    let num_input = input
        .iter()
        .map(|v| {
            v.split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let mut sum = 0;

    for nums in num_input {
        sum += *nums.last().unwrap();
        let mut curr = nums
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect::<Vec<i64>>();

        sum += curr.last().unwrap();

        while !(curr.iter().all(|val| *val == 0)) {
            curr = curr
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .collect::<Vec<i64>>();
            sum += curr.last().unwrap();
        }
    }

    sum as usize
}

pub fn second_part(input: &[String]) -> usize {
    let num_input = input
        .iter()
        .map(|v| {
            v.split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .rev()
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let mut sum = 0;
    for nums in num_input {
        sum += *nums.last().unwrap();
        let mut curr = nums
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect::<Vec<i64>>();

        sum += curr.last().unwrap();

        while !(curr.iter().all(|val| *val == 0)) {
            curr = curr
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .collect::<Vec<i64>>();
            sum += curr.last().unwrap();
        }
    }
    sum as usize
}

pub fn execute() {
    let input = &read_input("input/day9.txt");
    let parts = (first_part(input), (second_part(input)));
    println!("Day 9, part 1: {}", parts.0);
    println!("Day 9, part 2: {}", parts.1);
}
