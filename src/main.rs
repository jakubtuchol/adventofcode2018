#[macro_use]
extern crate clap;
extern crate itertools;

use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process::exit;

mod day_one;
mod day_three;
mod day_two;

fn main() {
    let available_days: Vec<fn()> = vec![run_day_one, run_day_two, run_day_three];
    let matches = App::new("Advent of Code")
        .version("0.1")
        .author("Jakub T.")
        .arg(
            Arg::with_name("days")
                .short("d")
                .value_name("DAYS")
                .help("Enter a comma-separated list of days")
                .takes_value(true)
                .required(true)
                .multiple(true),
        )
        .get_matches();

    for day in values_t!(matches, "days", usize).unwrap_or_else(|e| e.exit()) {
        let idx = day - 1;
        if idx > available_days.len() {
            exit(1);
        }
        available_days[idx]();
    }
}

fn run_day_one() {
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
}

fn run_day_two() {
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

fn run_day_three() {
    // Day Three
    let day_three_file = match File::open("data/day_three.txt") {
        Ok(f) => f,
        Err(e) => panic!(e),
    };
    let reader = BufReader::new(day_three_file);

    let boxes: Result<Vec<String>, _> = reader.lines().collect();
    let unwrapped_boxes = boxes.unwrap();
    println!(
        "Day three part one answer is {}",
        day_three::find_num_overlapping_spaces(unwrapped_boxes.clone())
    );
    println!(
        "Day three part two answer is {}",
        day_three::find_non_overlapping_claim(unwrapped_boxes.clone())
    );
}
