#![allow(dead_code)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = get_input("input.txt");
    let schematic = collect_schematic(input);

    // Part 1
    // let sum = get_part_sum(schematic.0, schematic.1);
    // println!("sum of part numbers: {:?}", sum);

    // Part 2
    let sum = get_gear_sum(schematic.0, schematic.1);
    println!("sum of part numbers: {:?}", sum);
}

#[derive(Debug)]
struct Number {
    start: usize,
    end: usize,
    value: i32,
    read: bool,
}

impl Number {
    fn new(index: usize, length: usize, value: &str) -> Self {
        Self {
            start: index - length,
            end: index - 1,
            value: value.to_owned().parse().unwrap(),
            read: false,
        }
    }
}

#[derive(Debug)]
struct Symbol(usize, usize, char);

fn get_input(file: &str) -> Vec<String> {
    BufReader::new(File::open(file).expect("input file not found"))
        .lines()
        .map(Result::unwrap)
        .collect()
}

fn collect_schematic(lines: Vec<String>) -> (Vec<Vec<Number>>, Vec<Symbol>) {
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut numbers: Vec<Vec<Number>> = Vec::new();

    let mut x = 0;
    for line in lines {
        let mut nums = Vec::new();

        let mut num = String::new();
        for (y, c) in line.char_indices() {
            // record a number, record a symbol, or build a number
            match c {
                '.' => {
                    if !num.is_empty() {
                        nums.push(Number::new(y, num.len(), &num));
                        num.clear();
                    }
                }
                '!'..='/' | ':'..='@' => {
                    if !num.is_empty() {
                        nums.push(Number::new(y, num.len(), &num));
                        num.clear();
                    }
                    symbols.push(Symbol(x, y, c));
                }
                '0'..='9' => num.push(c),
                _ => println!("triggered"),
            }
        }

        // reached the end of the line and a number has not been recorded
        if !num.is_empty() {
            nums.push(Number::new(line.len(), num.len(), &num));
        }

        numbers.push(nums);
        x += 1;
    }

    (numbers, symbols)
}

// Part 1
fn get_part_sum(mut numbers: Vec<Vec<Number>>, symbols: Vec<Symbol>) -> i32 {
    // println!("{:?}, {:?}", numbers, symbols); //Debugging
    let mut sum = 0;
    for sym in symbols {
        for (x, y) in get_range(sym) {
            for num in &mut numbers[x] {
                if num.start <= y && y <= num.end && !num.read {
                    num.read = true;
                    sum += num.value;
                }
            }
        }
    }
    sum
}

// Part 2
fn get_gear_sum(mut numbers: Vec<Vec<Number>>, symbols: Vec<Symbol>) -> i32 {
    // println!("{:?}, {:?}", numbers, symbols); //Debugging
    let mut sum = 0;
    for sym in symbols {
        if sym.2 != '*' {
            continue;
        }

        let mut nums = Vec::new();

        for (x, y) in get_range(sym) {
            for num in &mut numbers[x] {
                if num.start <= y && y <= num.end && !num.read {
                    num.read = true;
                    nums.push(num.value);
                }
            }
        }

        if nums.clone().into_iter().count() == 2 {
            sum += nums[0] * nums[1];
        }
    }
    sum
}

// Helper for symbol coverage
fn get_range(symbol: Symbol) -> Vec<(usize, usize)> {
    /*
        (-1,-1) (-1,0) (-1,+1)
        (0, -1) (0, 0) (0, +1)
        (+1,-1) (+1,0) (+1,+1)
    */
    vec![
        (symbol.0 - 1, symbol.1 - 1),
        (symbol.0 - 1, symbol.1),
        (symbol.0 - 1, symbol.1 + 1),
        (symbol.0, symbol.1 - 1),
        (symbol.0, symbol.1 + 1),
        (symbol.0 + 1, symbol.1 - 1),
        (symbol.0 + 1, symbol.1),
        (symbol.0 + 1, symbol.1 + 1),
    ]
}
