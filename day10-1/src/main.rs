use std::fs::File;
use std::io::{BufRead, BufReader};

#[inline(always)]
fn char_to_val(c: char) -> usize {
    match c {
        '(' | ')' => 0,
        '[' | ']' => 1,
        '{' | '}' => 2,
        '<' | '>' => 3,
        _ => 9999,
    }
}

#[inline(always)]
fn is_opening(c: char) -> bool {
    match c {
        '(' | '[' | '{' | '<' => true,
        _ => false,
    }
}

fn main() {
    let f = File::open("./input.txt").expect("Unable to open input file");
    let f = BufReader::new(f);

    let mut stack: Vec<usize> = Vec::new();

    //let open_chars = ['(', '[', '{', '<'];
    let closed_chars = [')', ']', '}', '>'];

    let points = [3u64, 57u64, 1197u64, 25137u64];
    let mut count: [u32; 4] = [0u32; 4];

    let mut current_line = 0;

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        current_line += 1;

        if line.trim().is_empty() {
            break;
        }

        let mut ch_pos = 0;

        for c in line.trim().chars() {
            ch_pos += 1;

            if is_opening(c) {
                stack.push(char_to_val(c));
                continue;
            }

            if stack.len() > 0 && char_to_val(c) != *stack.last().unwrap() {
                println!("Bracket mismatch on line {}, pos {}; Expected {}, found {}",
                    current_line,
                    ch_pos,
                    closed_chars[*stack.last().unwrap()],
                    c);
                count[char_to_val(c) as usize] += 1;
                break;
            }

            if stack.len() == 0 {
                println!("End of sequence while not in sequence on line {} - found {}",
                    current_line,
                    c);
                count[char_to_val(c)] += 1;
                break;
            }

            stack.pop();
        }

        if stack.len() != 0 {
            println!("Line {} is incomplete", current_line);
            stack.clear();
        }
    }

    let score = {
        let mut x: u64 = 0;
        
        for i in 0..4 {
            x += points[i] * count[i] as u64;
        }
        x
    };

    println!("Score Breakdown:\n) - {}\n] - {}\n}} - {}\n> - {}\nTotal Score: {}", 
    points[0] * count[0] as u64,
    points[1] * count[1] as u64,
    points[2] * count[2] as u64,
    points[3] * count[3] as u64,
    score);
}