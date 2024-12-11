use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("./input.txt").expect("Unable to open input file");
    let f = BufReader::new(f);

    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut risk: Vec<u8> = Vec::new();

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        
        if line.trim().is_empty() {
            break;
        }

        if line.len() != 100 {
            panic!("Bad heightmap data (x)");
        }    

        let nums: Vec<u8> = line.trim().chars().map(|c| c.to_digit(10).unwrap() as u8).collect();

        grid.push(nums);
    }

    if grid.len() != 100 {
        panic!("Bad heightmap data (y)");
    }

    for i in 0..100 {
        for j in 0..100 {
            let mut greater: bool = false;
            
            // above
            if i > 0 && grid[i][j] >= grid[i-1][j] {
                greater = true;
            }

            // left
            if j > 0 && grid[i][j] >= grid[i][j-1] {
                greater = true;
            }

            // right
            if j != 99 && grid[i][j] >= grid[i][j+1] {
                greater = true;
            }

            // below
            if i != 99 && grid[i][j] >= grid[i+1][j] {
                greater = true;
            }

            if !greater {
                println!("Found low point at ({},{}): {}", i+1, j+1, grid[i][j]+1);
                risk.push(grid[i][j] + 1);
            }
        }
    }

    let mut risk_total: u32 = 0;
    for i in 0..risk.len() {
        risk_total += risk[i] as u32;
    }

    println!("Total low points: {}, total risk: {}", risk.len(), risk_total);
}