use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let fil = File::open("./input.txt").expect("Unable to open input file");
    let fil = BufReader::new(fil);

    let mut fish: Vec<u8> = Vec::new();

    for line in fil.lines() {
        let line = line.expect("Unable to read line");
        
        if line.trim().is_empty() {
            break;
        }

        for x in line.split(",").map(|x| x.parse::<u8>().unwrap()) {
            fish.push(x);
        }
    }

    for _day in 1..=80 {
        let mut new_fish: u32 = 0;

        for i in 0..fish.len() {
            if fish[i] == 0 {
                fish[i] = 6;
                new_fish += 1;
                continue;
            }

            fish[i] -= 1;
        }

        for _i in 0..new_fish {
            fish.push(8);
        }
    }

    println!("There are {} fish after 80 days!", fish.len());
}