#![allow(dead_code)]
use num::Integer;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = get_input("input.txt");
    let path = build_path(input);
    let steps = get_steps(path.0, path.1, path.2);

    println!("{:?}", steps);
}

type Node = String;

type Path = HashMap<Node, (Node, Node)>;

fn get_input(file: &str) -> Vec<String> {
    BufReader::new(File::open(file).expect("input file not found"))
        .lines()
        .map(Result::unwrap)
        .filter(|line| line != "")
        .collect()
}

fn build_path(mut input: Vec<String>) -> (String, Path, Vec<Node>) {
    let mut path = Path::new();
    let directions = input.remove(0);
    let mut starts = Vec::new();

    for line in input {
        if line[2..3] == *"A" {
            starts.push(line[..3].to_owned());
        }
        path.insert(
            line[..3].to_owned(),
            (line[7..10].to_owned(), line[12..15].to_owned()),
        );
    }

    (directions, path, starts)
}

fn get_steps(directions: String, path: Path, starts: Vec<Node>) -> usize {
    let mut ends: Vec<usize> = Vec::new();

    for mut node in starts {
        let mut steps = 0;
        for dir in directions.chars().cycle() {
            // println!("{:?}, {:?}, {:?}", node, dir, path.get(&node).unwrap());
            node = match dir {
                'L' => path[&node].0.clone(),
                'R' => path[&node].1.clone(),
                _ => "".to_string(),
            };

            steps += 1;
            if node.chars().nth(2).unwrap() == 'Z' {
                ends.push(steps);
                break;
            }
        }
    }

    ends.iter().fold(1, |n1, n2| n1.lcm(n2))
}
