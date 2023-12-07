#![allow(dead_code)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::zip,
};

fn main() {
    // Part 1
    // let races = get_races("input.txt");
    // let wins = get_wins(races);
    // println!("Win margin: {:?}", wins);

    // Part 2
    let race = get_race("input.txt");
    println!("Win margin: {:?}", race.wins());
}

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    fn wins(&self) -> u64 {
        let mut wins = 0;
        for s in 1..self.time {
            if s * (self.time - s) > self.distance {
                wins += 1;
            }
        }
        wins
    }
}

//Part 1
fn get_races(file: &str) -> Vec<Race> {
    let input: Vec<String> = BufReader::new(File::open(file).expect("input file not found"))
        .lines()
        .map(Result::unwrap)
        .filter(|line| line != "")
        .collect();

    let times: Vec<u64> = input[0]
        .split(" ")
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();

    let distances: Vec<u64> = input[1]
        .split(" ")
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();

    // group first time and distance, second time and distance, etc.
    zip(times, distances)
        .map(|(t, d)| Race::new(t, d))
        .collect()
}

fn get_wins(races: Vec<Race>) -> u64 {
    races.iter().map(|r| r.wins()).product()
}

// Part 2
fn get_race(file: &str) -> Race {
    let input: Vec<String> = BufReader::new(File::open(file).expect("input file not found"))
        .lines()
        .map(Result::unwrap)
        .filter(|line| line != "")
        .collect();

    let time: u64 = input[0][10..].replace(" ", "").parse::<u64>().unwrap();
    let distance: u64 = input[1][10..].replace(" ", "").parse::<u64>().unwrap();

    Race::new(time, distance)
}
