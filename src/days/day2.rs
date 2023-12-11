use std::cmp::max;

use crate::utils::read_input;

fn first_part(input: Vec<String>) -> usize {
    let mut possible_games = 0;
    'outer: for line in input.into_iter() {
        let (game, rest) = line.split_once(':').unwrap();
        let rounds = rest.split(';').collect::<Vec<&str>>();
        for round in &rounds {
            let (mut red, mut green, mut blue): (i32, i32, i32) = (12, 13, 14);
            let cubes = round.split(',').collect::<Vec<&str>>();
            for cube in cubes {
                let cube = cube.trim();
                let (count, color) = cube.split_once(' ').unwrap();
                let count = count.parse::<i32>().unwrap();
                match color {
                    "red" => red -= count,
                    "green" => green -= count,
                    "blue" => blue -= count,
                    _ => {}
                }
            }
            if red < 0 || green < 0 || blue < 0 {
                continue 'outer;
            }
        }

        let (_, game_id) = game.split_once(' ').unwrap();
        possible_games += game_id.parse::<usize>().unwrap();
    }
    possible_games
}

fn second_part(input: Vec<String>) -> i32 {
    let mut total = 0;

    for line in input.into_iter() {
        let (_game, rest) = line.split_once(':').unwrap();
        let rounds = rest.split(';').collect::<Vec<&str>>();
        let (mut red, mut green, mut blue): (i32, i32, i32) = (0, 0, 0);
        for round in &rounds {
            let cubes = round.split(',').collect::<Vec<&str>>();
            for cube in cubes {
                let cube = cube.trim();
                let (count, color) = cube.split_once(' ').unwrap();
                let count = count.parse::<i32>().unwrap();
                match color {
                    "red" => {
                        red = max(red, count);
                    }
                    "green" => {
                        green = max(green, count);
                    }
                    "blue" => {
                        blue = max(blue, count);
                    }
                    _ => {}
                }
            }
        }
        let power = red * green * blue;
        total += power;
    }

    total
}

pub fn execute() {
    let input = &read_input("input/day2.txt");
    let parts = (first_part(input.clone()), (second_part(input.clone())));
    println!("Day 2, part 1: {}", parts.0);
    println!("Day 2, part 2: {}", parts.1);
}
