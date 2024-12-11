use std::fs::File;
use std::io::{BufRead, BufReader, Write};

const GRID_DIM: usize = 500;
const GRID_LEN: usize = GRID_DIM * GRID_DIM;

struct Node {
    g_score: usize,
    f_score_idx: usize,
    previous: Option<usize>,
    weight: usize,
    visited: bool,
}

#[inline(always)]
fn get_x(idx: usize) -> usize {
    idx % GRID_DIM
}

#[inline(always)]
fn get_y(idx: usize) -> usize {
    idx / GRID_DIM
}

#[inline(always)]
fn up(idx: usize) -> usize {
    if get_y(idx) > 0 {
        idx - GRID_DIM
    }
    else {
        usize::MAX
    }
}

#[inline(always)]
fn down(idx: usize) -> usize {
    if get_y(idx) < GRID_DIM - 1 {
        idx + GRID_DIM
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
    if get_x(idx) < GRID_DIM - 1 {
        idx + 1
    }
    else {
        usize::MAX
    }
}

#[inline(always)]
fn h(idx: usize) -> usize {
    let tmp_x: i32 = get_x(idx) as i32 - GRID_DIM as i32 - 1;
    let tmp_y: i32 = get_y(idx) as i32 - GRID_DIM as i32 - 1;
    (tmp_x.abs() + tmp_y.abs()) as usize
}

const DIR_NONE: usize = 0;
const DIR_LEFT: usize = 1;
const DIR_RIGHT: usize = 2;

#[derive(Debug)]
struct ListItem {
    next: Option<usize>,
    prev: Option<usize>,
    value: usize,
    node_idx: usize,
}

#[derive(Debug)]
struct MinList {
    min_idx: Option<usize>,
    start_idx: Option<usize>,
    list: Vec<ListItem>,
}

impl MinList {
    fn new() -> MinList {
        MinList {
            min_idx: Some(GRID_LEN - 1),
            start_idx: Some(0),
            list: Vec::new(),
        }
    }

    // See if this node is where it is supposed to be
    fn check_loc(&mut self, idx: usize) -> (bool, Option<usize>, Option<usize>) {
        let mut dir: usize = DIR_NONE;

        // Positions
        let mut compare_pos: Option<usize> = None;
        let mut last_pos: Option<usize> = None;

        // See if this value is greater than the previous one
        if !self.list[idx].prev.is_none() && self.list[idx].value > self.list[self.list[idx].prev.unwrap()].value {
            dir = DIR_LEFT;
            compare_pos = self.list[idx].prev;
        }
        // See if this value is less than the next one
        else if !self.list[idx].next.is_none() && self.list[idx].value < self.list[self.list[idx].next.unwrap()].value {
            dir = DIR_RIGHT;
            compare_pos = self.list[idx].next;
        }

        // If this value is in the right place, return current (prev, next)
        if dir == DIR_NONE {
            return (false, self.list[idx].prev, self.list[idx].next);
        }

        loop {
            match dir {
                DIR_LEFT => {
                    if self.list[idx].value > self.list[compare_pos.unwrap()].value {
                        last_pos = compare_pos;
                        compare_pos = self.list[compare_pos.unwrap()].prev;

                        if compare_pos.is_none() {
                            break;
                        }
                    }
                    else {
                        break;
                    }
                },
                DIR_RIGHT => {
                    if self.list[idx].value < self.list[compare_pos.unwrap()].value {
                        last_pos = compare_pos;
                        compare_pos = self.list[compare_pos.unwrap()].next;

                        if compare_pos.is_none() {
                            break;
                        }
                    }
                    else {
                        break;
                    }
                },
                _ => {},
            }
        }

        // returns (updated_flag, prev, next) for given idx
        if dir == DIR_LEFT {
            (true, compare_pos, last_pos)
        }
        else {
            (true, last_pos, compare_pos)
        }        
    }

    // Link a node into the list 
    fn link_nodes(&mut self, src_idx: usize, prev_idx: Option<usize>, next_idx: Option<usize>) {
        self.list[src_idx].prev = prev_idx;
        if prev_idx.is_some() {            
            self.list[prev_idx.unwrap()].next = Some(src_idx);
        }
        else {
            self.start_idx = Some(src_idx);
        }
        
        self.list[src_idx].next = next_idx;
        if next_idx.is_some() {
            self.list[next_idx.unwrap()].prev = Some(src_idx);
        }
        else {
            self.min_idx = Some(src_idx);
        }

    }

    // Unlink a node from list
    fn unlink_node(&mut self, idx: usize) {
        let old_prev = self.list[idx].prev;
        let old_next = self.list[idx].next;

        self.list[idx].prev = None;
        self.list[idx].next = None;

        //println!("Unlinking idx {}, prev {:?}, next {:?}", idx, old_prev, old_next);

        if old_prev.is_some() && old_next.is_some() {
            self.list[old_prev.unwrap()].next = old_next;
            self.list[old_next.unwrap()].prev = old_prev;
        }
        else if old_prev.is_none() && old_next.is_some() {
            self.list[old_next.unwrap()].prev = None;
            self.start_idx = old_next;
        }
        else if old_prev.is_some() && old_next.is_none() {
            self.list[old_prev.unwrap()].next = None;
            self.min_idx = old_prev;
        }
        else if old_prev.is_none() && old_next.is_none() {
            self.start_idx = None;
            self.min_idx = None;
        }
    }
    
    fn update_f_score(&mut self, idx: usize, new_f_score: usize) {
        self.list[idx].value = new_f_score;

        let location_check = self.check_loc(idx);

        if location_check.0 {
            // Unlink this node
            self.unlink_node(idx);

            // Relink with new coords
            self.link_nodes(idx, location_check.1, location_check.2);
        }
    }

    fn get_lowest_f(&mut self) -> Option<usize> {
        if self.min_idx.is_some() {
            Some(self.list[self.min_idx.unwrap()].node_idx)
        }
        else {
            None
        }
    }
}

fn main() {
    let f = File::open("./input.txt").expect("Unable to open input file");
    let f = BufReader::new(f);

    let mut grid: Vec<Node> = Vec::new();
    let mut minlist: MinList = MinList::new();

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        
        if line.trim().is_empty() {
            continue;
        }
        
        let weights: Vec<usize> = line.trim().chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
        
        for i in 0..weights.len() {
            grid.push(Node {
                g_score: usize::MAX,
                f_score_idx: usize::MAX,
                previous: None,
                weight: weights[i],
                visited: false,
            });
        }

        if GRID_DIM == 500 {
            for addend in 1..=4 {
                for i in 0..weights.len() {
                    let new_weight = {
                        if weights[i] + addend > 9 {
                            (weights[i] + addend) - 9
                        }
                        else {
                            weights[i] + addend
                        }
                    };
                    grid.push(Node {
                        g_score: usize::MAX,
                        f_score_idx: usize::MAX,
                        previous: None,
                        weight: new_weight,
                        visited: false,
                    });
                }
            }
        }
    }

    let orig_grid_len = grid.len();

    if GRID_DIM == 500 {
        for addend in 1..=4 {
            for i in 0..orig_grid_len {
                let new_weight = {
                    if grid[i].weight + addend > 9 {
                        (grid[i].weight + addend) - 9
                    }
                    else {
                        grid[i].weight + addend
                    }
                };
                grid.push(Node {
                    g_score: usize::MAX,
                    f_score_idx: usize::MAX,
                    previous: None,
                    weight: new_weight,
                    visited: false,
                });
            }
        }
    }

    // Build the minlist
    grid[GRID_LEN - 1].f_score_idx = 0;
    minlist.list.push(ListItem { node_idx: GRID_LEN - 1, prev: None, next: Some(1), value: usize::MAX });

    let mut ci: usize = 1;

    for i in (1..=GRID_LEN - 2).rev() {
        grid[i].f_score_idx = ci;
        minlist.list.push(ListItem { node_idx: i, prev: Some(ci - 1), next: Some(ci + 1), value: usize::MAX });
        ci += 1;
    }

    grid[0].f_score_idx = GRID_LEN - 1;
    minlist.list.push(ListItem { node_idx: 0, prev: Some(GRID_LEN - 2), next: None, value: h(0) });
    minlist.min_idx = Some(minlist.list.len() - 1);

    grid[0].g_score = 0;

    println!("grid[{}].weight == {}, minlist -> start {}, min {}", grid.len()-1, grid[grid.len()-1].weight, minlist.start_idx.unwrap(), minlist.min_idx.unwrap());

    
    {
        let mut output = File::create("grid_out.txt").expect("Unable to open output file");
        for i in 0..grid.len() {
            if i > 0 && i % GRID_DIM == 0 {
                write!(output, "\n");
            }
            let _ = output.write(grid[i].weight.to_string().as_bytes());
        }
    }

    while minlist.min_idx != None {
        let lf = minlist.get_lowest_f();
        if lf.is_none() {
            println!("No path found");
            break;
        }

        let cur_idx = lf.unwrap();

        grid[cur_idx].visited = true;
        minlist.unlink_node(grid[cur_idx].f_score_idx);

        //println!("Lowest f index {}, unvisited index {}", cur_idx, cur_unvisited_idx);

        // Are we the target node?
        if cur_idx == GRID_LEN - 1 {
            println!("Cost of the shortest path is {}", grid[cur_idx].g_score);
            break;
        }

        //println!("({}, {}) == {}", get_x(cur_idx), get_y(cur_idx), grid[cur_idx].g_score);

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
                minlist.update_f_score(grid[*n].f_score_idx, grid[*n].g_score + h(*n));
            }
        }

        //println!("Current cell index: {}, minlist -> start {:?}, min {:?}", cur_idx, minlist.start_idx, minlist.min_idx);
    }    
}