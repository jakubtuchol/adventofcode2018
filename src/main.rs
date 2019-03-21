#[macro_use]
extern crate itertools;

use std::clone;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod day_one;
mod day_two;

fn main() {
    // Day One
    let day_one_file = match File::open("data/day_one.txt") {
        Ok(f) => f,
        Err(e) => panic!(e),
    };
    let reader = BufReader::new(day_one_file);

    let frequencies: Result<Vec<String>, _> = reader.lines().collect();

    let unwrapped_frequencies = frequencies.unwrap();

    println!(
        "Day one part one answer is {}",
        day_one::calculate_final_frequency(unwrapped_frequencies.clone())
    );
    println!(
        "Day one part two answer is {}",
        day_one::find_repeated_frequency(unwrapped_frequencies.clone())
    );

    // Day Two
    let day_two_file = match File::open("data/day_two.txt") {
        Ok(f) => f,
        Err(e) => panic!(e),
    };
    let reader = BufReader::new(day_two_file);

    let boxes: Result<Vec<String>, _> = reader.lines().collect();
    let unwrapped_boxes = boxes.unwrap();
    println!(
        "Day two part one answer is {}",
        day_two::get_box_list_checksum(unwrapped_boxes.clone())
    );
    println!(
        "Day two part two answer is {}",
        day_two::get_common_letters(unwrapped_boxes.clone()).unwrap(),
    );
}
