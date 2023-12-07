use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = get_input("input.txt");
    // println!("{:?}", input); // Debugging
    let sum = get_calibration_sum(input);
    println!("sum of calibration values: {:?}", sum);
}

fn get_calibration_sum(lines: Vec<String>) -> i32 {
    lines
        .into_iter()
        .map(|line| {
            // get the first and last number in the line and parse to an i32
            let mut value = String::new();
            value.push(line.chars().filter(|c| c.is_numeric()).next().unwrap());
            value.push(line.chars().filter(|c| c.is_numeric()).last().unwrap());
            return value.parse::<i32>().unwrap();
        })
        .sum()
}

fn get_input(file: &str) -> Vec<String> {
    BufReader::new(File::open(file).expect("input file not found"))
        .lines()
        .map(Result::unwrap)
        .map(|l| map_to_digits(l))
        .collect()
}

fn map_to_digits(mut line: String) -> String {
    let digit_mappings: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    // look for words and inert their corresponding digit in between
    // eg. eightwothree -> e8ight2wot3hree
    for word in digit_mappings.keys() {
        // track indexes that have already been inserted to avoid infite loops
        let mut inserted: HashSet<usize> = HashSet::new();

        while let Some(i) = line.find(word) {
            if inserted.contains(&i) {
                break;
            };

            line.insert(i + 1, *digit_mappings.get(word).unwrap());
            inserted.insert(i);
        }
    }

    line
}
