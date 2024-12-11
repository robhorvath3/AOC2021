use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("./input.txt").expect("Unable to open input file");
    let f = BufReader::new(f);

    let mut grid: Vec<[u32; 1000]> = Vec::new();

    
    for _i in 0..1000 {
        grid.push([0u32; 1000]);
    }

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        if line.trim().is_empty() {
            continue;
        }

        let start_end_pair: Vec<&str> = line.split(" -> ").collect();
        
        let start: Vec<usize> = start_end_pair[0].split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        let end: Vec<usize> = start_end_pair[1].split(',').map(|x| x.parse::<usize>().unwrap()).collect();

        if start[0] == end[0] {
            if start[1] <= end[1] {
                for y in start[1]..=end[1] {
                    grid[start[0]][y] += 1;
                }
            }
            else {
                for y in (end[1]..=start[1]).rev() {
                    grid[start[0]][y] += 1;
                }
            }
        }
        else if start[1] == end[1] {
            if start[0] <= end[0] {
                for x in start[0]..=end[0] {
                    grid[x][start[1]] += 1;
                }
            }
            else {
                for x in (end[0]..=start[0]).rev() {
                    grid[x][start[1]] += 1;
                }
            }
        }
        else {
            let mut pos_x: i32 = start[0] as i32;
            let step_x: i32 = {
                if start[0] > end[0] {
                    -1
                }
                else {
                    1
                }
            };

            let mut pos_y: i32 = start[1] as i32;
            let step_y: i32 = {
                if start[1] > end[1] {
                    -1
                }
                else {
                    1
                }
            };
            
            let mut isfinal: bool = false;

            loop {
                grid[pos_x as usize][pos_y as usize] += 1;
                
                if isfinal {
                    break;
                }

                pos_x += step_x;
                pos_y += step_y;

                if pos_x == end[0] as i32 || pos_y == end[1] as i32 {
                    isfinal = true;
                }
            }
        }
    }

    let double_intersections: u32 = {
        let mut r: u32 = 0;
        for i in 0..1000 {
            for j in 0..1000 {
                if grid[i][j] >= 2 {
                    r += 1;
                }
            }
        }
        r
    };

    println!("There are {} points where 2 or more lines intersect", double_intersections);
}