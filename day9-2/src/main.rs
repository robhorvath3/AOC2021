use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone)]
struct Point {
    val: u8,
    x: u8,
    y: u8,
    visited: bool,
}

fn main() {
    let f = File::open("./input.txt").expect("Unable to open input file");
    let f = BufReader::new(f);

    let mut grid: Vec<Vec<Point>> = Vec::new();
    let mut risk: Vec<Point> = Vec::new();

    let mut grid_x: u8 = 0;
    let mut grid_y: u8 = 0;

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        
        if line.trim().is_empty() {
            break;
        }

        if line.len() != 100 {
            panic!("Bad heightmap data (x)");
        }    

        let mut row: Vec<Point> = Vec::new();
        
        for d in line.trim().chars().map(|c| c.to_digit(10).unwrap() as u8) {
            row.push(Point { val: d, x: grid_x, y: grid_y, visited: false});
            grid_x += 1;
        }

        grid.push(row);
        grid_x = 0;
        grid_y += 1;
    }

    if grid.len() != 100 {
        panic!("Bad heightmap data (y)");
    }

    for i in 0..100 {
        for j in 0..100 {
            let mut greater: bool = false;
            
            // above
            if i > 0 && grid[i][j].val >= grid[i-1][j].val {
                greater = true;
            }

            // left
            if j > 0 && grid[i][j].val >= grid[i][j-1].val {
                greater = true;
            }

            // right
            if j != 99 && grid[i][j].val >= grid[i][j+1].val {
                greater = true;
            }

            // below
            if i != 99 && grid[i][j].val >= grid[i+1][j].val {
                greater = true;
            }

            if !greater {
                println!("Found low point at ({},{}): {}", j, i, grid[i][j].val+1);
                risk.push(grid[i][j]);
            }
        }
    }

    fn count_basin(grid: &mut Vec<Vec<Point>>, p: Point) -> u32 {
        let mut tot: u32 = 0;

        //println!("Entering ({}, {})", p.x, p.y);

        if grid[p.y as usize][p.x as usize].visited {
            return 0;
        }

        grid[p.y as usize][p.x as usize].visited = true;

        // left
        if p.x > 0 && grid[p.y as usize][p.x as usize - 1].val < 9 {
            tot += count_basin(grid, Point { val: 0, x: p.x-1, y: p.y, visited: false });
        }

        // up
        if p.y > 0 && grid[p.y as usize - 1][p.x as usize].val < 9 {
            tot += count_basin(grid, Point { val: 0, x: p.x, y: p.y-1, visited: false });
        }

        // right
        if p.x < 99 && grid[p.y as usize][p.x as usize + 1].val < 9 {
            tot += count_basin(grid, Point { val: 0, x: p.x+1, y: p.y, visited: false });
        }

        // down
        if p.y < 99 && grid[p.y as usize + 1][p.x as usize].val < 9 {
            tot += count_basin(grid, Point { val: 0, x: p.x, y: p.y+1, visited: false });
        }

        //println!("Leaving ({}, {})", p.x, p.y);
        tot + 1
    }

    let mut basin_size: Vec<u32> = Vec::new();

    for i in 0..risk.len() {
        let bs = count_basin(&mut grid, risk[i]);
        basin_size.push(bs);
        println!("Basin {}, centered at Low Point ({}, {}) consists of {} units of terrain", i+1, risk[i].x, risk[i].y, bs);
    }

    basin_size.sort();
    basin_size.reverse();

    let mut product: u64 = 1u64;

    for b in basin_size.iter().take(3) {
        println!("Top 3 basin of size {} found", *b);
        product *= *b as u64;
    }

    println!("Total product of top 3 basins: {}", product);
}