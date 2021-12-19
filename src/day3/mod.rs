use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Mul;
use std::str::FromStr;

pub fn run() {
    part_1("src/day3/input.txt");
}

const BIT: char = '1';
const ZERO: char = '0';

struct Count {
    bit: i32,
    zero: i32,
}

impl Count {
    fn increment_bit(&mut self) {
        println!("Increment BIT");
        self.bit += 1
    }
    fn increment_zero(&mut self) {
        println!("Increment ZERO");
        self.zero += 1
    }

    fn max(&self) -> i32 {
        return if self.bit > self.zero {
            println!("BIT wins");
            1
        } else {
            println!("ZERO wins");
            0
        }
    }

    fn min(&self) -> i32 {
        if self.bit > self.zero { 0 } else {  1 }
    }
}

fn prepare_map(first_line_len: usize) -> HashMap<usize, Count> {
    let mut map: HashMap<usize, Count> = HashMap::new();
    let x = || Count { bit: 0, zero: 0 };
    for i in 0..first_line_len {
        map.insert(i, x());
    }
    map
}

fn part_1(file_name: &str) -> i64 {
    let instructions = read_file(file_name);
    let first_line_len = instructions[0].len();
    let mut map: HashMap<usize, Count> = prepare_map(first_line_len);

    increment_values(instructions, &mut map);
    let mut sorted_elements = sort_count_map(&mut map);
    let (to_gamma_rate_binary_string, gamma_rate) = find_gamma_rate(&mut sorted_elements);
    let (to_ipsilon_rate_binary_string, ipsilon_rate) = find_ipsilon_rate(&mut sorted_elements);

    println!("Gamma binary string: {}, decimal: {} ", to_gamma_rate_binary_string, gamma_rate);
    println!("Ipsilon binary string: {}, decimal: {} ", to_ipsilon_rate_binary_string, ipsilon_rate);
    let result = gamma_rate * ipsilon_rate;
    println!("Result: {:?} Completely in rust!", result);
    return result;
}

fn increment_values(instructions: Vec<String>, map: &mut HashMap<usize, Count>) {
    let mut line_counter = 0;
    for single_line in instructions.iter() {
        line_counter += 1;
        println!("line #{}, value: {}", line_counter, single_line);

        let mut sign_counter: usize = 0;
        for i in single_line.chars() {
            let position_counter = map.get_mut(&sign_counter).unwrap();
            match i {
                BIT => position_counter.increment_bit(),
                ZERO => position_counter.increment_zero(),
                _ => {}
            }
            sign_counter += 1;
        }
    }
}

fn sort_count_map(map: &mut HashMap<usize, Count>) -> Vec<(&usize, &Count)> {
    let mut sorted_elements: Vec<(&usize, &Count)> = map.iter().collect();
    println!("#Checking elements order in map");
    sorted_elements.sort_by(|a, b| a.0.cmp(&b.0));
    for (i, _) in &sorted_elements {
        print!(" {} ", i)
    }
    println!("\n");
    sorted_elements
}

fn find_gamma_rate(sorted_elements: &mut Vec<(&usize, &Count)>) -> (String, i64) {
    let to_gamma_rate_binary_string = sorted_elements.iter()
        .map(|(k, count)| {
            println!("Element: #{}", k);
            count
        })
        .fold(String::from(""), |mut acc, x| {
            let string = &*x.max().to_string();
            acc.push(char::from_str(string).unwrap());
            acc
        });

    let gamma_rate = i64::from_str_radix((&*to_gamma_rate_binary_string), 2).unwrap();
    (to_gamma_rate_binary_string, gamma_rate)
}

fn find_ipsilon_rate(sorted_elements: &mut Vec<(&usize, &Count)>) -> (String, i64) {
    let to_ipsilon_rate_binary_string = sorted_elements.iter()
        .map(|(k, count)| {
            println!("Element: #{}", k);
            count
        })
        .fold(String::from(""), |mut acc, x| {
            let string = &*x.min().to_string();
            acc.push(char::from_str(string).unwrap());
            acc
        });

    let ipsilon_rate = i64::from_str_radix(&*to_ipsilon_rate_binary_string, 2).unwrap();
    (to_ipsilon_rate_binary_string, ipsilon_rate)
}

fn read_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap())
        .collect()
}

fn count_position_2(file_name: &str) {
    println!("Result: {}. Completely in rust!", "todo");
}

#[cfg(test)]
mod day2_test {
    use super::*;

    #[test]
    fn day3_part_1(){
        let result = part_1("src/day3/test_input.txt");
        assert_eq!(198, result);
    }
}
