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

    let points = [1u64, 2u64, 3u64, 4u64];

    let mut current_line = 0;
    let mut scores: Vec<u64> = Vec::new();

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        current_line += 1;

        if line.trim().is_empty() {
            break;
        }

        let mut ch_pos = 0;
        let mut discard: bool = false;

        for c in line.trim().chars() {
            ch_pos += 1;

            if is_opening(c) {
                stack.push(char_to_val(c));
                continue;
            }

            if stack.len() > 0 && char_to_val(c) != *stack.last().unwrap() {
                println!("Bracket mismatch on line {}, pos {}; Expected {}, found {}, discarding",
                    current_line,
                    ch_pos,
                    closed_chars[*stack.last().unwrap()],
                    c);
                discard = true;
                break;
            }

            if stack.len() == 0 {
                println!("End of sequence while not in sequence on line {} - found {}, discarding",
                    current_line,
                    c);
                discard = true;
                break;
            }

            stack.pop();
        }

        if stack.len() != 0 && !discard {
            scores.push({
                let mut score_sum: u64 = 0;
                print!("Line {} is incomplete - Needed {} matches: ", current_line, stack.len());
                while stack.len() > 0 {
                    score_sum *= 5;
                    score_sum += points[stack.pop().unwrap()] as u64;
                }
                print!("Line score is {}\n", score_sum);
                score_sum
            });
            
            stack.clear();
        }
        else if discard {
            stack.clear();
        }
    }

    scores.sort();

    println!("The size of scores is {}, and the middle score is {}", scores.len(), scores[(scores.len() - 1) / 2]);
}