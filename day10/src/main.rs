#![allow(dead_code)]
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = get_input("input.txt");
    let mut pipes = get_pipes(&input);
    let closed_pipes = closed_pipe_system(&pipes);

    // Part 1
    // println!("furthest point: {:?}", closed_pipes.len() / 2);

    // Part 2
    println!("enclosed: {:?}", get_enclosed(&mut pipes, &closed_pipes));
}

/* valid pipe:
    .....
    .S-7.
    .|.|.
    .L-J.
    .....
*/
#[derive(Debug, Clone, Copy)]
enum Pipe {
    V,
    H,
    Ne,
    Nw,
    Sw,
    Se,
    G,
    S,
}
use Pipe::*;

impl Pipe {
    fn from(s: &str) -> Self {
        match s {
            "|" => Self::V,
            "-" => Self::H,
            "L" => Self::Ne,
            "J" => Self::Nw,
            "7" => Self::Sw,
            "F" => Self::Se,
            "." => Self::G,
            "S" => Self::S,
            _ => panic!("invalid pipe {:?}", s),
        }
    }

    fn valid_right(&self, other: &Self) -> bool {
        match (self, other) {
            (H | Ne | Se | S, H | Nw | Sw | S) => true,
            (_, _) => false,
        }
    }

    fn valid_left(&self, other: &Self) -> bool {
        match (self, other) {
            (H | Nw | Sw | S, H | Ne | Se | S) => true,
            (_, _) => false,
        }
    }

    fn valid_up(&self, other: &Self) -> bool {
        match (self, other) {
            (V | Ne | Nw | S, V | Se | Sw | S) => true,
            (_, _) => false,
        }
    }

    fn valid_down(&self, other: &Self) -> bool {
        match (self, other) {
            (V | Se | Sw | S, V | Ne | Nw | S) => true,
            (_, _) => false,
        }
    }

    fn valid(&self, other: &Self, dir: &str) -> bool {
        match dir {
            "right" => self.valid_right(other),
            "left" => self.valid_left(other),
            "up" => self.valid_up(other),
            "down" => self.valid_down(other),
            _ => panic!("invalid direction"),
        }
    }
}

fn get_input(file: &str) -> Vec<String> {
    BufReader::new(File::open(file).expect("input file not found"))
        .lines()
        .map(Result::unwrap)
        .collect()
}

fn get_pipes(input: &Vec<String>) -> Vec<Vec<Pipe>> {
    input
        .iter()
        .map(|line| line.chars().map(|c| Pipe::from(&c.to_string())).collect())
        .collect()
}

fn closed_pipe_system(pipes: &Vec<Vec<Pipe>>) -> HashSet<(usize, usize)> {
    let mut start: (usize, usize) = (0, 0);
    for i in 0..pipes.len() {
        for j in 0..pipes[i].len() {
            if let Pipe::S = pipes[i][j] {
                start = (i, j);
            }
        }
    }

    let mut queue: Vec<(usize, usize)> = vec![start];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while let Some((i, j)) = queue.pop() {
        let pipe = pipes[i][j];
        visited.insert((i, j));
        if let Pipe::G = pipe {
            continue;
        }

        if i > 0 && !visited.contains(&(i - 1, j)) {
            if pipe.valid(&pipes[i - 1][j], "up") {
                queue.push((i - 1, j));
            }
        }
        if i < pipes.len() - 1 && !visited.contains(&(i + 1, j)) {
            if pipe.valid(&pipes[i + 1][j], "down") {
                queue.push((i + 1, j));
            }
        }
        if j > 0 && !visited.contains(&(i, j - 1)) {
            if pipe.valid(&pipes[i][j - 1], "left") {
                queue.push((i, j - 1));
            }
        }
        if j < pipes[i].len() - 1 && !visited.contains(&(i, j + 1)) {
            if pipe.valid(&pipes[i][j + 1], "right") {
                queue.push((i, j + 1));
            }
        }
    }

    visited
}

fn get_enclosed(pipes: &mut Vec<Vec<Pipe>>, closed: &HashSet<(usize, usize)>) -> usize {
    let mut enclosed = 0;

    for i in 0..pipes.len() {
        for j in 0..pipes[i].len() {
            pipes[i][j] = if closed.contains(&(i, j)) {
                match pipes[i][j] {
                    V | Ne | Nw | S => V,
                    _ => H,
                }
            } else {
                G
            };
        }
    }

    for i in 0..pipes.len() {
        let mut contained = 0;

        for j in 0..pipes[i].len() {
            if let V = pipes[i][j] {
                contained += 1;
            } else if let G = pipes[i][j] {
                enclosed += contained % 2;
            }
        }
    }

    enclosed
}
