use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Mul;
use std::str::FromStr;

pub fn run() {
    count("src/day3/input.txt");
}

fn prepare_map() -> HashMap<usize, Count> {
    let mut map: HashMap<usize, Count> = HashMap::new();
    let x = || Count { bit: 0, zero: 0 };
    map.insert(0, x());
    map.insert(1, x());
    map.insert(2, x());
    map.insert(3, x());
    map.insert(4, x());
    map.insert(5, x());
    map.insert(6, x());
    map.insert(7, x());
    map.insert(8, x());
    map.insert(9, x());
    map.insert(10, x());
    map.insert(11, x());
    map
}

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
        if self.bit > self.zero {
            println!("BIT wins");
            return 1;
        } else {
            println!("ZERO wins");
            return 0;
        }
    }

    fn min(&self) -> i32 {
        if self.bit > self.zero { 0 } else {  1 }
    }
}

const BIT: char = '1';
const ZERO: char = '0';

fn count(file_name: &str) {
    let instructions = read_file(file_name);
    let mut map: HashMap<usize, Count> = prepare_map();

    let mut line_counter = 0;
    for single_line in instructions.iter() {
        line_counter += 1;
        println!("line #{}, value: {}", line_counter, single_line);
        let len = single_line.len();

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

    let mut sorted_elements: Vec<(&usize, &Count)> = map.iter().collect();
    println!("#Checking elements order in map");
    sorted_elements.sort_by(|a, b| a.0.cmp(&b.0));
    for (i, _) in &sorted_elements {
        print!(" {} ", i)
    }
    println!("\n");

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

    println!("Gamma binary string: {}, decimal: {} ", to_gamma_rate_binary_string, gamma_rate);
    println!("Ipsilon binary string: {}, decimal: {} ", to_ipsilon_rate_binary_string, ipsilon_rate);
    println!("Result: {:?} Completely in rust!", gamma_rate.mul(ipsilon_rate));
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
