use std::fmt::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::{FromStr, SplitWhitespace};

pub fn run() {
    count_position("src/day2/input.txt");
}

fn read_file(filename: &str) -> Vec<Instruction> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap())
        .map(|line| parse_to_instruction(line))
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap())
        .collect()
}

fn parse_to_instruction(line: String) -> Result<Instruction, Error> {
    let split: Vec<&str> = line.split(" ").collect();
    let direction = Direction::from_str(split[0]).unwrap();
    let steps = i32::from_str(split[1]).unwrap();
    let instruction = Instruction { direction, steps };
    Result::Ok(instruction)
}

struct Instruction {
    direction: Direction,
    steps: i32,
}

#[derive(PartialEq)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_lowercase() {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

fn count_position(file_name: &str) {
    let instructions = read_file(file_name);
    let mut horizontal = Vec::new();
    let mut vertical = Vec::new();
    for i in instructions {
        match i.direction {
            Direction::Forward => { horizontal.push(i.steps) }
            Direction::Down => { vertical.push(i.steps ) }
            Direction::Up => { vertical.push(i.steps * -1) }
        }
    }

    let horizontal_index: i32 = horizontal.iter().sum();
    let vertical_index: i32 = vertical.iter().sum();

    println!("Result of second half day: {}. Completely in rust!", horizontal_index * vertical_index);
}
