use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let fil = File::open("./input.txt").expect("Unable to open input file");
    let fil = BufReader::new(fil);

    let mut output_values: Vec<Vec<String>> = Vec::new();
    
    for line in fil.lines() {
        let line = line.expect("Unable to read line");
        
        if line.trim().is_empty() {
            break;
        }

        let pipe_split: Vec<String> = line.split('|').map(|s| String::from(s.trim())).collect();

        if pipe_split.len() != 2 {
            panic!("Malformed input!");
        }

        let o_vals: Vec<String> = pipe_split[1].split_whitespace().map(|s| String::from(s)).collect();

        output_values.push(o_vals);
    }

    let mut digit_count_1_4_7_8 = 0;

    for i in 0..output_values.len() {
        for j in 0..output_values[i].len() {
            match output_values[i][j].len() {
                2 | 4 | 3 | 7 => digit_count_1_4_7_8 += 1,
                _ => {}
            }
        }
    }

    println!("The digits 1, 4, 7, or 8 appear {} times in {} output displays", digit_count_1_4_7_8, output_values.len());
}