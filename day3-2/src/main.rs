use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("./input.txt").expect("Unable to open input file");
    let f = BufReader::new(f);

    let oxygen_rating: u32;
    let co2_rating: u32;
    let mut values: Vec<String> = Vec::new();
    let file_lines = f.lines();

    for line in file_lines {
        //let line = line.expect("Unable to read line");
        values.push(line.expect("Unable to read line"));
    }

    oxygen_rating = {
        let mut pos = 0;
        let mut tmp_values: Vec<String> = values.clone();
        let mut tmp_values2: Vec<String>;

        loop {
            tmp_values2 = get_most_common(tmp_values, pos);
            if tmp_values2.len() == 1 {
                break;
            }
            else {
                tmp_values = tmp_values2.clone();
                pos += 1;
            }
        }

        if tmp_values2.len() != 1 {
            panic!("Unable to locate oxygen generator rating");
        }

        let mut r: u32 = 0;
        let bytes = tmp_values2[0].trim().as_bytes();

        for i in 0..12 {
            if bytes[i] == b'1' {
                r += 1 << (11-i);
            }            
        }
        r
    };

    co2_rating = {
        let mut pos = 0;
        let mut tmp_values: Vec<String> = values.clone();
        let mut tmp_values2: Vec<String>;

        loop {
            tmp_values2 = get_least_common(tmp_values, pos);
            if tmp_values2.len() == 1 {
                break;
            }
            else {
                tmp_values = tmp_values2.clone();
                pos += 1;
            }
        }

        if tmp_values2.len() != 1 {
            panic!("Unable to locate oxygen generator rating");
        }

        let mut r: u32 = 0;
        let bytes = tmp_values2[0].trim().as_bytes();

        for i in 0..12 {
            if bytes[i] == b'1' {
                r += 1 << (11-i);
            }            
        }
        r
    };

    println!("Diagnostics report oxygen generator rating as {} and CO2 scrubber rating as {}; life support rating is {}", oxygen_rating, co2_rating, oxygen_rating * co2_rating);
}

fn get_most_common(vals: Vec<String>, position: usize) -> Vec<String> {
    let mut rv: Vec<String> = Vec::new();
    let mut ones_count: usize = 0;
    let mut char_match: u8 = b'0';

    for i in 0..vals.len() {
        let bytes = vals[i].trim().as_bytes().to_vec();
        if position >= bytes.len() {
            panic!("Trying to read bytes from an invalid position {} where length is {}", position, bytes.len());
        }

        if bytes[position] == b'1' {
            ones_count += 1;
        }
    }

    if ones_count >= vals.len() - ones_count {
        char_match = b'1';
    }

    for i in 0..vals.len() {
        let bytes = vals[i].trim().as_bytes().to_vec();
        if position >= bytes.len() {
            panic!("Trying to read bytes from an invalid position {} where length is {}", position, bytes.len());
        }

        if bytes[position] == char_match {
            rv.push(vals[i].clone());
        }
    }

    rv
}

fn get_least_common(vals: Vec<String>, position: usize) -> Vec<String> {
    let mut rv: Vec<String> = Vec::new();
    let mut zero_count: usize = 0;
    let mut char_match: u8 = b'0';

    for i in 0..vals.len() {
        let bytes = vals[i].trim().as_bytes().to_vec();
        if position >= bytes.len() {
            panic!("Trying to read bytes from an invalid position {} where length is {}", position, bytes.len());
        }

        if bytes[position] == b'0' {
            zero_count += 1;
        }
    }

    if zero_count > vals.len() - zero_count {
        char_match = b'1';
    }

    for i in 0..vals.len() {
        let bytes = vals[i].trim().as_bytes().to_vec();
        if position >= bytes.len() {
            panic!("Trying to read bytes from an invalid position {} where length is {}", position, bytes.len());
        }

        if bytes[position] == char_match {
            rv.push(vals[i].clone());
        }
    }

    rv
}