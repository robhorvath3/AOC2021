use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("./input.txt").expect("Unable to open input file");
    let f = BufReader::new(f);

    let mut horizontal: u32 = 0;
    let mut depth: u32 = 0;
    
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let keyvalpair: Vec<&str> = line.split(" ").collect();

        if keyvalpair.len() != 2 {
            panic!("Mismatched key value pair!");
        }

        let magnitude: u32 = keyvalpair.get(1).unwrap_or(&"0").trim().parse::<u32>().unwrap();

        match keyvalpair.get(0).unwrap().trim() {
            "forward" => {
                horizontal += magnitude;
            },
            "down" => {
                depth += magnitude;
            },
            "up" => {
                depth -= magnitude;
            },
            _ => {}
        }
    }

    println!("After following the course, the horizontal position is {} and the depth is {}, and multiplying them yields {}", horizontal, depth, horizontal * depth);
}
