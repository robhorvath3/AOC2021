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

    let mut max_x: usize = 0;
    let mut max_y: usize = 0;

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
        println!("Folding on {}-axis at {} == {}", 
            { if splits[i].axis == 0 { "y" } else { "x" } },
            { if splits[i].axis == 0 { "x" } else { "y" } }, 
            splits[i].value);

        for j in 0..points.len() {
            if points[j][splits[i].axis] > splits[i].value {
                points[j][splits[i].axis] = (2 * splits[i].value) - points[j][splits[i].axis];
            }
        }
    }
    
    let mut y_index: HashMap<usize, usize> = HashMap::new();
    let mut rows: Vec<Vec<usize>> = Vec::new();

    for i in 0..points.len() {
        if points[i][0] > max_x {
            max_x = points[i][0];
        }
        if points[i][1] > max_y {
            max_y = points[i][1];
        }
        
        if !y_index.contains_key(&points[i][1]) {
            rows.push(Vec::new());
            y_index.insert(points[i][1], rows.len()-1);
        }
        rows[*y_index.get(&points[i][1]).expect("Unable to locate final set of points")].push(points[i][0]);
    }

    for i in 0..rows.len() {
        rows[i].sort();
    }

    for y in 0..=max_y {
        let yy = y_index.get(&y);
        
        if yy.is_none() {
            print!("\n");
            continue;
        }
        let yy = *yy.expect("Unable to locate final set of points - yy");

        for x in 0..=max_x {
            if rows[yy].contains(&x) {
                print!("#");
            }
            else {
                print!(" ");
            }
        }
        print!("\n");
    }
}