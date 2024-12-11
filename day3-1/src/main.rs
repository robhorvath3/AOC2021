use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("./input.txt").expect("Unable to open input file");
    let f = BufReader::new(f);

    let mut gamma_rate: u32 = 0;
    let mut epsilon_rate: u32 = 0;

    let mut zero_count: Vec<u32> = vec![0; 12];
    let mut one_count: Vec<u32> = vec![0; 12];

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let bytes = line.trim().as_bytes().to_vec();

        if bytes.len() != 12 {
            panic!("Bad diagnostic data");
        }

        for i in 0..12 {
            match bytes.get(i).unwrap() {
                b'0' => {
                    zero_count[i] += 1;
                },
                b'1' => {
                    one_count[i] += 1;
                },
                _ => {}
            }
        }
    }

    for i in 0..12 {
        if zero_count[i] > one_count[i] {
            epsilon_rate += 1 << (11-i);
        }
        else {
            gamma_rate += 1 << (11-i);
        }
    }

    println!("Diagnostics report gamma rate as {} and epsilon rate as {}; power consumption is {}", gamma_rate, epsilon_rate, gamma_rate * epsilon_rate);
}
