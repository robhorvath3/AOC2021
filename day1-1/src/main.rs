use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("./input.txt").expect("Unable to open input file");
    let f = BufReader::new(f);

    let mut inc: u32 = 0;
    let mut count: u32 = 0;
    let mut prev: u32 = 0;

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let current = line.parse::<u32>().unwrap();
        if count > 0 && current > prev {
            inc += 1;
        }
        prev = current;
        count += 1;
    }

    println!("There are {} measurements that are larger than the previous measurement out of {} measurements", inc, count);
}
