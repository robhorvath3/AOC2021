use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone)]
struct Point {
    val: u8,
    row: u8,
    col: u8,
    flashed: bool,
}

fn main() {
    let f = File::open("./input.txt").expect("Unable to open input file");
    let f = BufReader::new(f);

    let mut grid: Vec<Vec<Point>> = Vec::new();

    let mut grid_row: u8 = 0;
    let mut grid_col: u8 = 0;
    
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        
        if line.trim().is_empty() {
            break;
        }

        if line.len() != 10 {
            panic!("Bad heightmap data (x)");
        }    

        let mut row: Vec<Point> = Vec::new();
        
        for d in line.trim().chars().map(|c| c.to_digit(10).unwrap() as u8) {
            row.push(Point { val: d, row: grid_row, col: grid_col, flashed: false });
            grid_col += 1;
        }

        grid.push(row);
        grid_col = 0;
        grid_row += 1;
    }

    if grid.len() != 10 {
        panic!("Bad heightmap data (y)");
    }

    let mut counter: u64 = 0;

    for day in 1..=100 {
        println!("Day {}", day);
        
        inc_by_one(&mut grid);
        
        for row in 0..10 {
            for col in 0..10 {
                if grid[row][col].val > 9 && !grid[row][col].flashed {
                    flash(&mut grid, row, col, &mut counter);
                }
            }
        }

        clear_flashed_and_zero(&mut grid);
    }

    #[inline(always)]
    fn clear_flashed_and_zero(grid: &mut Vec<Vec<Point>>) {
        for row in 0..10 {
            for col in 0..10 {
                grid[row][col].flashed = false;
                if grid[row][col].val > 9 {
                    grid[row][col].val = 0;
                }
            }
        }
    }

    #[inline(always)]
    fn inc_by_one(grid: &mut Vec<Vec<Point>>) {
        for row in 0..10 {
            for col in 0..10 {
                grid[row][col].val += 1;
            }
        }
    }

    fn flash(grid: &mut Vec<Vec<Point>>, row: usize, col: usize, counter: &mut u64) {
        if grid[row][col].flashed {
            return;
        }

        if grid[row][col].val > 9 {
            grid[row][col].flashed = true;
            println!("Flashing ({}, {})", row, col);
            *counter += 1;
        }
        else {
            return;
        }

        // Up & Left
        if row > 0 && col > 0 && !grid[row-1][col-1].flashed {
            grid[row-1][col-1].val += 1;
            if grid[row-1][col-1].val > 9 {
                flash(grid, row-1, col-1, counter);
            }
        }

        // Up
        if row > 0 && !grid[row-1][col].flashed {
            grid[row-1][col].val += 1;
            if grid[row-1][col].val > 9 {
                flash(grid, row-1, col, counter);
            }
        }

        // Up & Right
        if row > 0 && col < 9 && !grid[row-1][col+1].flashed {
            grid[row-1][col+1].val += 1;
            if grid[row-1][col+1].val > 9 {
                flash(grid, row-1, col+1, counter);
            }
        }

        // Right
        if col < 9 && !grid[row][col+1].flashed {
            grid[row][col+1].val += 1;
            if grid[row][col+1].val > 9 {
                flash(grid, row, col+1, counter);
            }
        }

        // Down & Right
        if row < 9 && col < 9 && !grid[row+1][col+1].flashed {
            grid[row+1][col+1].val += 1;
            if grid[row+1][col+1].val > 9 {
                flash(grid, row+1, col+1, counter);
            }
        }

        // Down
        if row < 9 && !grid[row+1][col].flashed {
            grid[row+1][col].val += 1;
            if grid[row+1][col].val > 9 {
                flash(grid, row+1, col, counter);
            }
        }

        // Down & Left
        if row < 9 && col > 0 && !grid[row+1][col-1].flashed {
            grid[row+1][col-1].val += 1;
            if grid[row+1][col-1].val > 9 {
                flash(grid, row+1, col-1, counter);
            }
        }

        // Left
        if col > 0 && !grid[row][col-1].flashed {
            grid[row][col-1].val += 1;
            if grid[row][col-1].val > 9 {
                flash(grid, row, col-1, counter);
            }
        }
    }

    println!("Counted {} flashes over 100 days", counter);
}