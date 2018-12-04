use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

mod day_one;

fn main() {
    // Day One
    let day_one_file = match File::open("data/day_one.txt") {
        Ok(f) => f,
        Err(e) => panic!(e),
    };
    let reader = BufReader::new(day_one_file);

    let mut frequencies: Vec<String> = Vec::new();

    for line in reader.lines() {
        let frequency = line.unwrap();
        frequencies.push(frequency.trim().to_string());
    }

    println!("Day one part one answer is {}", day_one::calculate_final_frequency(frequencies.as_slice()))
}
