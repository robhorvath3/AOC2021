use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

struct Cave {
    visited: bool,
    index: usize,
    start: bool,
    end: bool,
    name: String,
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
                        visited: false, 
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

    

    fn count_path(map: &mut HashMap<String, usize>, caves: &mut Vec<Cave>, paths: &Vec<Vec<usize>>, current_cave: usize) -> u64 {
        let end_cave = *map.get("end").expect("Unable to locate end cave");
        
        if current_cave == end_cave {
            return 1;
        }

        caves[current_cave].visited = true;

        let mut pc: u64 = 0;

        for p in &paths[current_cave] {
            if !caves[*p].visited || (caves[*p].visited && caves[*p].name == caves[*p].name.to_uppercase()) {
                pc += count_path(map, caves, &paths, *p);
            }
        }

        caves[current_cave].visited = false;
        pc
    }

    let start_cave = *map.get("start").expect("Unable to locate start cave");
    let path_count: u64 = count_path(&mut map, &mut caves, &paths, start_cave);
    println!("Counted {} paths through the maze from {:?}", path_count, paths);
}