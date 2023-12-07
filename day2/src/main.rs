#![allow(dead_code)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // Part 1
    // let sum = get_id_sum("input.txt");
    // println!("sum of ids: {:?}", sum);

    // Part 2
    let sum = get_power_sum("input.txt");
    println!("sum of powers: {:?}", sum);
}

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<Set>,
    min: Set,
}

impl Game {
    fn from(input: &str) -> Self {
        let id_end = input.find(":").unwrap();
        let sets: Vec<Set> = input.split(";").map(|s| Set::from(s)).collect();

        let mut largest_red = 0;
        let mut largest_green = 0;
        let mut largest_blue = 0;
        for Set(r, g, b) in sets.clone() {
            if r > largest_red {
                largest_red = r
            }
            if g > largest_green {
                largest_green = g
            }
            if b > largest_blue {
                largest_blue = b
            }
        }

        Self {
            id: input[5..id_end].parse::<i32>().unwrap(),
            sets,
            min: Set(largest_red, largest_green, largest_blue),
        }
    }

    fn validate(&self, red: i32, green: i32, blue: i32) -> bool {
        for Set(r, g, b) in &self.sets {
            if r > &red || g > &green || b > &blue {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone, Copy)]
struct Set(i32, i32, i32);

impl Set {
    fn from(input: &str) -> Self {
        let red = match input.find("red") {
            Some(i) => input[i - 3..i - 1].replace(" ", "").parse::<i32>().unwrap(),
            None => 0,
        };
        let green = match input.find("green") {
            Some(i) => input[i - 3..i - 1].replace(" ", "").parse::<i32>().unwrap(),
            None => 0,
        };
        let blue = match input.find("blue") {
            Some(i) => input[i - 3..i - 1].replace(" ", "").parse::<i32>().unwrap(),
            None => 0,
        };
        Self(red, green, blue)
    }
}

// Part 1
fn get_id_sum(file: &str) -> i32 {
    BufReader::new(File::open(file).expect("input file not found"))
        .lines()
        .map(Result::unwrap)
        .map(|line| Game::from(&line))
        .filter(|game| game.validate(12, 13, 14))
        .fold(0, |sum, game| sum + game.id)
}

// Part 2
fn get_power_sum(file: &str) -> i32 {
    BufReader::new(File::open(file).expect("input file not found"))
        .lines()
        .map(Result::unwrap)
        .map(|line| Game::from(&line))
        .fold(0, |sum, game| sum + (game.min.0 * game.min.1 * game.min.2))
}
