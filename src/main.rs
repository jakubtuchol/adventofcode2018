use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    // Day One
    let day_one_file = match File::open("data/day_one.txt") {
        Ok(f) => f,
        Err(e) => panic!(e),
    };
    let reader = BufReader::new(day_one_file);

    let frequencies: Vec<&str> = Vec::new();

    for line in reader.lines() {
        let frequency = line.unwrap();
        frequencies.push(&frequency.clone().trim());
    }

}
