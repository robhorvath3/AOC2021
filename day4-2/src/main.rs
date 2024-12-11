use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Score {
    row: [u16; 5],
    col: [u16; 5],
}

#[derive(Debug)]
struct Board {
    row: [u32; 100],
    col: [u32; 100],
    marked: [bool; 100],
    score: Score,
    winner: bool,
}

impl Board {
    fn new() -> Board {
        let b = Board {
            row: [9999; 100],
            col: [9999; 100],
            marked: [false; 100],
            score: Score { 
                row: [0; 5], 
                col: [0; 5], 
            },
            winner: false,
        };
        b
    }
}

impl Default for Board {
    fn default() -> Board {
        Board {
            row: [9999; 100],
            col: [9999; 100],
            marked: [false; 100],
            score: Score { 
                row: [0; 5], 
                col: [0; 5], 
            },
            winner: false,
        }
    }
}

fn main() {
    let f = File::open("./input.txt").expect("Unable to open input file");
    let f = BufReader::new(f);

    let lines: Vec<String> = f.lines().map(|l| l.unwrap()).collect();
    let moves: Vec<usize> = lines[0].split(",").map(|x| x.parse().unwrap()).collect();
    let mut boards: Vec<Board> = Vec::new();
    
    let mut board = Board::new();
    let mut row: usize = 0;
    let mut col: usize = 0;
    let mut cur_line: usize = 0;

    for line in lines {
        cur_line += 1;
        println!("Current Line #{}: {}", cur_line, line);

        if cur_line < 3 {
            continue;
        }

        if line.trim().is_empty() {
            boards.push(board);
            board = Board::new();
            row = 0;
            col = 0;
            continue;
        }

        println!("Working row: {}, col: {}", row, col);

        for i in line.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .take(5) {
                board.row[i] = row as u32;
                board.col[i] = col as u32;
                col += 1;            
            }
        
        col = 0;
        row += 1;        
    }
    
    for x in moves {
        for i in 0..boards.len() {
            if boards[i].col[x] != 9999 {
                boards[i].marked[x] = true;
                let cur_col = boards[i].col[x] as usize;
                let cur_row = boards[i].row[x] as usize;
                /*
                if i == 0 {
                    println!("Current move: {}, board: {}, row: {}, col: {}", x, i, cur_row, cur_col);
                }
                */

                boards[i].score.col[cur_col] += 1 << cur_row;
                boards[i].score.row[cur_row] += 1 << cur_col;

                for r in 0..boards[i].score.row.len() {                        
                    if boards[i].score.row[r] == 0b11111 && boards[i].winner == false {
                        //println!("Current board: {}, row: {}, score: {}", i, r, boards[i].score.row[r]);
                        let mut unmarked_sum: u32 = 0;
                        
                        for j in 0..boards[i].row.len() {
                            if boards[i].row[j] != 9999 && boards[i].marked[j as usize] == false {
                                unmarked_sum += j as u32;
                            }
                        }
                        println!("Winner on board {:?}, the unmarked sum is {}, the number just called was {} and the factor is {}", i, unmarked_sum, x, unmarked_sum * x as u32);
                        println!("Winner Found!");
                        boards[i].winner = true;
                    }
                }

                for c in 0..boards[i].score.col.len() {
                    if boards[i].score.col[c] == 0b11111 && boards[i].winner == false {
                        //println!("Current board: {}, col: {}, score: {}", i, c, boards[i].score.col[c]);
                        let mut unmarked_sum: u32 = 0;
                        
                        for j in 0..boards[i].row.len() {
                            if boards[i].row[j] != 9999 && boards[i].marked[j as usize] == false {
                                unmarked_sum += j as u32;
                            }
                        }
                        println!("Winner on board {:?}, the unmarked sum is {}, the number just called was {} and the factor is {}", i, unmarked_sum, x, unmarked_sum * x as u32);
                        println!("Winner Found!");
                        boards[i].winner = true;
                    }
                }
            }
        }
    }
}