use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let fil = File::open("./input.txt").expect("Unable to open input file");
    let fil = BufReader::new(fil);

    let mut days: [u64; 9] = [0u64; 9];

    for line in fil.lines() {
        let line = line.expect("Unable to read line");
        
        if line.trim().is_empty() {
            break;
        }

        for i in line.split(",").map(|x| x.parse::<u8>().unwrap()) {
            days[i as usize] += 1;
        }
    }

    for _day in 1..=256 {
        let new_fish = days[0];

        for i in 0..=7 {
            days[i] = days[i + 1];            
        }

        days[6] += new_fish;
        days[8] = new_fish;
    }

    let total_fish = {
        let mut tf: u64 = 0;
        for i in 0..=8 {
            tf += days[i as usize];
        }
        tf
    };

    println!("There are {} fish after 256 days!", total_fish);
}