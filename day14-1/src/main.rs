use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let f = File::open("./input.txt").expect("Unable to open input file");
    let f = BufReader::new(f);

    let mut map: HashMap<String, String> = HashMap::new();
    let mut current_str: String = String::from("");

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
            current_str = String::from(line.trim());                        
        }
        else {
            let tmp: Vec<&str> = line.trim().split(" -> ").collect();
            let chars: Vec<char> = tmp[0].chars().collect();
            map.insert(String::from(tmp[0]), format!("{}{}", tmp[1].trim(), chars[1]));
        }
    }

    println!("Step 0, len == {}", current_str.len());

    for step in 1..=10 {

        /*
        if step > 3 {
            break;
        }
        */

        //println!("Begin Step {}, Len: {}, String: {}", step, current_str.len(), current_str);
        
        let tmp = current_str.clone();

        current_str.clear();
        let chars: Vec<char> = tmp.chars().collect();
        let mut str_out: Vec<char> = Vec::new();

        for i in 0..chars.len()-1 {
            let s: String = format!("{}{}", chars[i], chars[i+1]);
            if map.contains_key(&s) {
                if i == 0 {
                    str_out.push(chars[i]);
                }

                let outchars: Vec<char> = map.get(&s).expect("Unable to locate key").chars().collect();
                for c in outchars {
                    str_out.push(c);
                }
                continue;
            }
        }

        current_str = String::from_iter(str_out);
        //println!("End Step {}, Len: {}, String: {}", step, current_str.len(), current_str);
        println!("Step {}, len == {}", step, current_str.len());
    }

    let mut letter_count: HashMap<char, usize> = HashMap::new();
    let mut min_char: char = 'A';
    let mut min_count: usize = 0;
    let mut max_char: char = 'Z';
    let mut max_count: usize = 0;

    for c in current_str.chars() {
        let mut new_count: usize = 0;

        if letter_count.contains_key(&c) {
            new_count = *letter_count.get(&c).expect("Unable to access key count index") + 1;
            letter_count.remove(&c);
            letter_count.insert(c, new_count);
        }
        else {
            letter_count.insert(c, new_count + 1);
        }                
    }

    for (key, count) in &letter_count {
        if *count < min_count || min_count == 0 {
            min_count = *count;
            min_char = *key;
        }
        else if min_char == *key {
            min_count = *count;
        }

        if *count > max_count {
            max_count = *count;
            max_char = *key;
        }
    }

    println!("Most Common Element: {} - {}, Least Common Element: {} - {}, Difference: {}",
        max_char, max_count, min_char, min_count, max_count - min_count);
    println!("{:?}", letter_count);
}