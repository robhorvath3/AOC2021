use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

fn main() {
    let f = File::open("./input.txt").expect("Unable to open input file");
    let f = BufReader::new(f);

    let mut dq: VecDeque<u32> = VecDeque::new();

    let mut inc_count: u32 = 0;
    let mut val_count: u32 = 0;
    let mut window_count: u32 = 0;
    let mut done: bool = false;

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let cur_val = line.parse::<u32>().unwrap_or_else(
            |_| {
                done = true;
                0
            }
        );
        
        if done {
            break;
        }

        val_count += 1;
        dq.push_back(cur_val);

        if dq.len() == 4 {
            window_count += 1;
            let prev = dq.get(0).unwrap() + dq.get(1).unwrap() + dq.get(2).unwrap();
            let curr = dq.get(1).unwrap() + dq.get(2).unwrap() + dq.get(3).unwrap();

            if curr > prev {
                inc_count += 1;
            }

            dq.pop_front();
        }        
    }

    println!("After comparing {} measurement windows, there are {} windows larger than the previous measurement window out of {} measurements", window_count + 1, inc_count, val_count);
}
