#![allow(dead_code)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = get_input("input.txt");
    let almanac = Almanac::from(input);

    println!(
        "lowest destination: {:?}",
        almanac.map_seeds().into_iter().min().unwrap()
    );
}

#[derive(Debug)]
struct Almanac {
    seed_ranges: Vec<Range>,
    mappings: Vec<Vec<Mapping>>,
}

impl Almanac {
    fn from(mut input: Vec<String>) -> Self {
        let seed_ranges: Vec<i64> = input
            .remove(0)
            .split(' ')
            .filter_map(|line| line.parse().ok())
            .collect();

        let seed_ranges: Vec<Range> = seed_ranges
            .chunks(2)
            .map(|s| Range {
                source: s[0],
                length: s[1],
            })
            .collect();

        // line 2 not needed
        input.remove(0);

        let mut mappings: Vec<Vec<Mapping>> = Vec::new();
        mappings.push(Vec::new());

        let mut category = 0;
        for line in input {
            // reached a new category
            if line.contains(':') {
                mappings.push(Vec::new());
                category += 1;
                continue;
            }

            // separate the line into destination, start and length of range
            let line: Vec<i64> = line.split(' ').filter_map(|n| n.parse().ok()).collect();
            // println!("{:?}", line);

            mappings[category].push(Mapping {
                range: Range {
                    source: line[1],
                    length: line[2],
                },
                dest: line[0],
            });
        }

        Self {
            seed_ranges,
            mappings,
        }
    }

    fn map_seeds(&self) -> Vec<i64> {
        let mut destinations = Vec::new();

        for range in &self.seed_ranges {
            for seed in (range.source..range.source + range.length).rev() {
                let mut dest = seed;
                // keep mapping the start seed until the end
                for mapping in &self.mappings {
                    dest = Self::map_with(dest, mapping);
                }

                // reduction to unrequired data saved
                if &dest < destinations.last().unwrap_or(&i64::MAX) {
                    destinations.push(dest);
                }
            }
        }

        destinations
    }

    fn map_with(seed: i64, mapping: &Vec<Mapping>) -> i64 {
        for map in mapping {
            if map.range.source <= seed && seed <= map.range.source + map.range.length {
                return seed + map.dest - map.range.source;
            }
        }

        seed
    }
}

#[derive(Debug, Clone, Copy)]
struct Mapping {
    range: Range,
    dest: i64,
}

#[derive(Debug, Clone, Copy)]
struct Range {
    source: i64,
    length: i64,
}

fn get_input(file: &str) -> Vec<String> {
    BufReader::new(File::open(file).expect("input file not found"))
        .lines()
        .map(Result::unwrap)
        .filter(|line| line != "")
        .collect()
}
