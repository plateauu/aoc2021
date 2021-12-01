use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() {
    count("src/day1/input.txt")
}

#[derive(PartialEq)]
enum Direction { Increased, Decreased }

fn count(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let depths: Vec<i32> = reader.lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap())
        .map(|line| line.parse())
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap())
        .collect();

    let mut prev = depths.first().unwrap();
    let x: Vec<Direction> = depths.iter().skip(1)
        .map(|item| {
            let pair = (item, if item >= prev { Direction::Increased } else { Direction::Decreased });
            prev = item;
            pair.1
        })
        .filter(|item| item == &Direction::Increased)
        .collect();

    println!("Result {}. Completely in rust!", x.len());
}
