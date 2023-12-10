#![allow(dead_code)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = get_input("input.txt");
    println!("sum: {}", get_final(input))
}

type History = Vec<Vec<i32>>;

fn get_input(file: &str) -> Vec<String> {
    BufReader::new(File::open(file).expect("input file not found"))
        .lines()
        .map(Result::unwrap)
        .collect()
}

fn get_final(input: Vec<String>) -> i32 {
    let mut sum = 0;

    for line in input.iter() {
        let history: History = generate_history(line);
        let mut level = history.len() - 1;
        let mut running_final = 0;

        while level > 0 {
            // Part 1
            // running_final += history[level - 1].first().unwrap();

            // Part 2
            running_final = history[level - 1].first().unwrap() - running_final;
            level -= 1;
        }

        sum += running_final;
    }

    sum
}

fn generate_history(line: &str) -> History {
    let mut history: History = vec![line
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()];
    let mut level = 0;

    while history[level].iter().any(|x| x != &0) {
        let line = &history[level];
        let mut new_line: Vec<i32> = Vec::new();
        for i in 0..line.len() - 1 {
            new_line.push(line[i + 1] - line[i]);
        }
        history.push(new_line);
        level += 1;
    }

    history
}
