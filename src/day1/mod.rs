use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() {
    count_first("src/day1/input.txt");
    count_second("src/day1/input.txt");
}

#[derive(PartialEq)]
enum Direction { Increased, Decreased }

fn count_first(filename: &str) {
    let depths = read_file(filename);
    let x = get_increased_direction(depths);
    println!("Result {}. Completely in rust!", x.len());
}

fn get_increased_direction(depths: Vec<i32>) -> Vec<Direction> {
    let mut prev = depths.first().unwrap();
    let x: Vec<Direction> = depths.iter().skip(1)
        .map(|item| {
            let pair = (item, if item > prev { Direction::Increased } else { Direction::Decreased });
            prev = item;
            pair.1
        })
        .filter(|item| item == &Direction::Increased)
        .collect();
    x
}

fn read_file(filename: &str) -> Vec<i32> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap())
        .map(|line| line.parse())
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap())
        .collect()
}

fn count_second(file_name: &str) {
    let depths = read_file(file_name);
    let len = depths.len();
    let mut result: Vec<i32> = Vec::new();

    for i in 0..(len - 2) {
        let first = depths.get(i);
        let second = depths.get(i + 1);
        let third = depths.get(i + 2);
        let sum = vec![first, second, third].iter()
            .map(|x| x.unwrap())
            .sum();
        result.insert(i, sum)
    }
    let x = get_increased_direction(result);
    println!("Result of second half day: {}. Completely in rust!", x.len());
}
