use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

struct Split {
    axis: usize,
    value: usize,
}

fn main() {
    let f = File::open("./input.txt").expect("Unable to open input file");
    let f = BufReader::new(f);

    let mut points: Vec<Vec<usize>> = Vec::new();
    let mut splits: Vec<Split> = Vec::new();

    let mut blank_line: bool = false;

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        
        if line.trim().is_empty() {
            if !blank_line {
                blank_line = true;
                continue;
            }
            else {
                break;
            }
        }

        if !blank_line {        
            let point: Vec<usize> = line.trim().split(",").map(|x| x.parse::<usize>().unwrap()).collect();
            
            if point.len() != 2 {
                panic!("Point specified incorrectly");
            }

            points.push(point);
        }
        else {
            let tmp0: Vec<&str> = line.trim().split("=").collect();

            let split_dir = {
                let tmp1: Vec<&str> = tmp0[0].split_whitespace().collect();
                if tmp1[2].trim() == "x" {
                    0
                }
                else {
                    1
                }
            };
            
            let split_value = tmp0[1].parse::<usize>().unwrap();
            
            splits.push(Split { axis: split_dir, value: split_value });
        }
    }

    for i in 0..splits.len() {

        if i == 1 {
            break;
        }

        for j in 0..points.len() {
            if points[j][splits[i].axis] > splits[i].value {
                points[j][splits[i].axis] = (2 * splits[i].value) - points[j][splits[i].axis];
            }
        }
    }
    
    let mut points_remaining: HashMap<String, bool> = HashMap::new();

    for i in 0..points.len() {
        let s: String = format!("{}{}", points[i][0], points[i][1]);
        if !points_remaining.contains_key(&s) {
            points_remaining.insert(s, true);
        }
    }

    println!("Counted {} points remaining", points_remaining.len());
}