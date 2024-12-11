use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

struct Cave {
    visited: u64,
    index: usize,
    start: bool,
    end: bool,
    name: String,
    is_upper: bool,
}

fn main() {
    let f = File::open("./input.txt").expect("Unable to open input file");
    let f = BufReader::new(f);

    let mut caves: Vec<Cave> = Vec::new();
    let mut paths: Vec<Vec<usize>> = Vec::new();
    let mut map: HashMap<String, usize> = HashMap::new();

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        
        if line.trim().is_empty() {
            break;
        }

        let nodes: Vec<String> = line.trim().split("-").map(|s| String::from(s.trim())).collect();
        
        if nodes.len() != 2 {
            panic!("Path specified incorrectly");
        }

        for i in 0..2 {
            if !map.contains_key(&nodes[i]) {
                caves.push(
                    Cave { 
                        visited: 0, 
                        index: caves.len(), 
                        start: {
                            if nodes[i] == "start" {
                                true
                            }
                            else {
                                false
                            }
                        },
                        end: {
                            if nodes[i] == "end" {
                                true
                            }
                            else {
                                false
                            }
                        },
                        name: String::from(&nodes[i]),
                        is_upper: {
                            if nodes[i].to_uppercase() == nodes[i] {
                                true
                            }
                            else {
                                false
                            }
                        }
                    }
                );
                map.insert(nodes[i].clone(), caves.len()-1);
                paths.push(Vec::new());
            }
        }

        let left_index: usize = *map.get(&nodes[0] as &str).expect("Unable to get node index");
        let right_index: usize = *map.get(&nodes[1] as &str).expect("Unable to get node index");

        paths[left_index].push(right_index);
        paths[right_index].push(left_index);
    }

    let mut multi_visit: usize = 9999;

    fn count_path(map: &HashMap<String, usize>, caves: &mut Vec<Cave>, paths: &Vec<Vec<usize>>, current_cave: usize, multi_visit: &mut usize) -> u64 {
        
        if caves[current_cave].end {            
            return 1;
        }

        caves[current_cave].visited += 1;

        let mut pc: u64 = 0;

        if *multi_visit == 9999 {
            for p in &paths[current_cave] {
                if caves[*p].start {
                    continue;
                }
                if caves[*p].visited == 0 || caves[*p].is_upper {
                    pc += count_path(&map, caves, &paths, *p, multi_visit);
                }
            }
        }
        else {
            for p in &paths[current_cave] {
                if caves[*p].start {
                    continue;
                }
                if caves[*p].visited == 0 || caves[*p].is_upper {
                    pc += count_path(&map, caves, &paths, *p, multi_visit);
                }
                else if caves[*p].visited == 1 && *p == *multi_visit {
                    pc += count_path(&map, caves, &paths, *p, multi_visit);
                }
            }
        }

        caves[current_cave].visited -= 1;
        pc
    }

    let start_cave = *map.get("start").expect("Unable to locate start cave");
    let no_multi_count: u64 = count_path(&map, &mut caves, &paths, start_cave, &mut multi_visit);

    let mut path_count = no_multi_count;
    println!("Counted {} paths through the maze with no double visits", path_count);

    for i in 0..caves.len() {
        if !caves[i].is_upper && !caves[i].start && !caves[i].end {
            multi_visit = i;
            path_count += count_path(&map, &mut caves, &paths, start_cave, &mut multi_visit) - no_multi_count;
        }
    }

    println!("Counted {} paths through the maze from {:?}", path_count, paths);
}