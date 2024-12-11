use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let fil = File::open("./input.txt").expect("Unable to open input file");
    let fil = BufReader::new(fil);

    let mut crabs: Vec<u64> = Vec::new();
    let mut pos_min: u64 = 0;
    let mut pos_max: u64 = 0;

    for line in fil.lines() {
        let line = line.expect("Unable to read line");
        
        if line.trim().is_empty() {
            break;
        }

        for i in line.split(",").map(|x| x.parse::<u64>().unwrap()) {
            crabs.push(i);
        }
    }

    for i in 0..crabs.len() {
        if crabs[i] > pos_max {
            pos_max = crabs[i];
        }
        if pos_min == 0 || crabs[i] < pos_min {
            pos_min = crabs[i];
        }
    }

    let mut min_fuel_used: u64 = 0;
    let mut min_pos: u64 = 0;

    for i in pos_min..=pos_max {
        let mut fuel: u64 = 0;
        for j in 0..crabs.len() {
            fuel += i64::abs(i as i64 - crabs[j] as i64) as u64
        }
        if min_fuel_used == 0 || fuel < min_fuel_used {
            min_fuel_used = fuel;
            min_pos = i;
        }
    }

    println!("The minimum total fuel used is {} units at position {}", min_fuel_used, min_pos);
}