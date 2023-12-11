#![allow(dead_code)]
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = get_input("input.txt");
    let universe = build_universe(input);
    let distances = get_distances(universe.0, universe.1, universe.2);
    println!("{:?}", distances);
}

type Universe = Vec<Vec<char>>;

#[derive(Debug)]
struct Galaxy(usize, usize);

fn get_input(file: &str) -> Vec<Vec<char>> {
    BufReader::new(File::open(file).expect("input file not found"))
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().collect())
        .collect()
}

fn build_universe(input: Vec<Vec<char>>) -> (Vec<Galaxy>, HashSet<usize>, HashSet<usize>) {
    let mut galaxies: Vec<Galaxy> = Vec::new();
    let mut h_empties: HashSet<usize> = HashSet::new();
    let mut v_empties: HashSet<usize> = HashSet::new();

    // scan universe verticaly & horizontally
    for col in 0..input[0].len() {
        let mut found_galaxy = false;

        for row in 0..input.len() {
            // store empty rows
            if !input[row].contains(&'#') {
                h_empties.insert(row);
            }

            if input[row][col] == '#' {
                galaxies.push(Galaxy(row, col));
                found_galaxy = true;
            }
        }

        // store empty columns
        if !found_galaxy {
            v_empties.insert(col);
        }
    }

    (galaxies, h_empties, v_empties)
}

fn get_distances(galaxies: Vec<Galaxy>, rows: HashSet<usize>, cols: HashSet<usize>) -> usize {
    let mut distance = 0;

    for (i, galaxy) in galaxies.iter().enumerate() {
        for j in i + 1..galaxies.len() {
            // Calculate manhattan distance between galaxies
            distance += (galaxy.0 as isize - galaxies[j].0 as isize).abs() as usize
                + (galaxy.1 as isize - galaxies[j].1 as isize).abs() as usize;

            // Part 1
            // const SIZE: usize = 1;
            // Part 2
            const SIZE: usize = 999_999;
            // Simulate expansion of the universe
            for i in &rows {
                if galaxy.0 < *i && galaxies[j].0 > *i || galaxy.0 > *i && galaxies[j].0 < *i {
                    distance += SIZE;
                }
            }
            for i in &cols {
                if galaxy.1 < *i && galaxies[j].1 > *i || galaxy.1 > *i && galaxies[j].1 < *i {
                    distance += SIZE;
                }
            }
        }
    }

    distance
}
