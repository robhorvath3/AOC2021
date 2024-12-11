use std::fs::File;
use std::io::{BufRead, BufReader};

struct Node {
    g_score: usize,
    f_score: usize,
    previous: Option<usize>,
    weight: usize,
    visited: bool,
}

#[inline(always)]
fn get_x(idx: usize) -> usize {
    idx % 100
}

#[inline(always)]
fn get_y(idx: usize) -> usize {
    idx / 100
}

#[inline(always)]
fn up(idx: usize) -> usize {
    if get_y(idx) > 0 {
        idx - 100
    }
    else {
        usize::MAX
    }
}

#[inline(always)]
fn down(idx: usize) -> usize {
    if get_y(idx) < 99 {
        idx + 100
    }
    else {
        usize::MAX
    }
}

#[inline(always)]
fn left(idx: usize) -> usize {
    if get_x(idx) > 0 {
        idx - 1
    }
    else {
        usize::MAX
    }
}

#[inline(always)]
fn right(idx: usize) -> usize {
    if get_x(idx) < 99 {
        idx + 1
    }
    else {
        usize::MAX
    }
}

#[inline(always)]
fn h(idx: usize) -> usize {
    2*((99 - (idx % 100)) + (99 - (idx / 100)))
}

// Returns index into grid
fn find_lowest_f(grid: &Vec<Node>, list: &Vec<usize>) -> (usize, usize) {
    let mut lowest_f = usize::MAX;
    let mut idx_lf = usize::MAX;
    let mut lowest_i = usize::MAX;

    for i in 0..list.len() {
        if grid[list[i]].f_score < lowest_f {
            lowest_f = grid[list[i]].f_score;
            idx_lf = list[i];
            lowest_i = i;
        }
    }
    (idx_lf, lowest_i)
}

fn main() {
    let f = File::open("./input.txt").expect("Unable to open input file");
    let f = BufReader::new(f);

    let mut grid: Vec<Node> = Vec::new();
    let mut unvisited: Vec<usize> = Vec::new();

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        
        if line.trim().is_empty() {
            continue;
        }
        
        let weights: Vec<usize> = line.trim().chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
        
        for i in 0..weights.len() {
            grid.push(Node {
                g_score: usize::MAX,
                f_score: usize::MAX,
                previous: None,
                weight: weights[i],
                visited: false,
            });
        }
    }

    grid[0].g_score = 0;
    grid[0].f_score = h(0);

    for i in 0..grid.len() {
        unvisited.push(i);
    }

    while unvisited.len() > 0 {
        let (cur_idx, cur_unvisited_idx) = find_lowest_f(&grid, &unvisited);
        //println!("Lowest f index {}, unvisited index {}", cur_idx, cur_unvisited_idx);

        // Are we the target node?
        if cur_idx == 9999 {
            println!("Cost of the shortest path is {}", grid[cur_idx].g_score);
            break;
        }

        // Analyze unvisited neighbors
        let mut neighbors: Vec<usize> = Vec::new();

        // Down
        let down_idx = down(cur_idx);
        if down_idx < usize::MAX && !grid[down_idx].visited {
            neighbors.push(down_idx);
        }

        // Right
        let right_idx = right(cur_idx);
        if right_idx < usize::MAX && !grid[right_idx].visited {
            neighbors.push(right_idx);
        }

        // Left
        let left_idx = left(cur_idx);
        if left_idx < usize::MAX && !grid[left_idx].visited {
            neighbors.push(left_idx);
        }

        // Up
        let up_idx = up(cur_idx);
        if up_idx < usize::MAX && !grid[up_idx].visited {
            neighbors.push(up_idx);
        }

        for n in &neighbors {
            let g_tmp = grid[cur_idx].g_score + grid[*n].weight;

            if g_tmp < grid[*n].g_score {
                grid[*n].g_score = g_tmp;
                grid[*n].previous = Some(cur_idx);
                grid[*n].f_score = grid[*n].g_score + h(*n);
            }
        }

        grid[cur_idx].visited = true;
        unvisited.remove(cur_unvisited_idx);      
    }
}