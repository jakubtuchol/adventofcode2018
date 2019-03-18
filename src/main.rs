#[macro_use] extern crate itertools;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::clone;

mod day_one;
mod day_two;

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

    println!("Day one part one answer is {}", day_one::calculate_final_frequency(frequencies.as_slice()));
    println!("Day one part two answer is {}", day_one::find_repeated_frequency(frequencies.as_slice()));

    // Day Two
    let day_two_file = match File::open("data/day_two.txt") {
        Ok(f) => f,
        Err(e) => panic!(e),
    };
    let reader = BufReader::new(day_two_file);

    //let boxes: Vec<String> = reader.lines().map(|x| x.unwrap().clone()).collect();
    let boxes: Vec<String> = reader.lines().map(|x| x.unwrap().clone()).collect();
    let ref_boxes: Vec<&str> = boxes.iter().map(|x| &**x).collect();
    /*
    for line in reader.lines() {
        let box_id = line.unwrap();
        boxes.push(box_id.trim().clone());
    }
    */
    println!("Day two part one answer is {}", day_two::get_box_list_checksum(boxes));
}
