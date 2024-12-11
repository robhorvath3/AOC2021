use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let fil = File::open("./input.txt").expect("Unable to open input file");
    let fil = BufReader::new(fil);

    let mut digit_maps: Vec<Vec<u8>> = Vec::new();
    let mut output_values: Vec<Vec<u8>> = Vec::new();
    
    fn string_to_map(map: &str) -> u8 {
        let mut val: u8 = 0u8;

        for c in map.as_bytes() {
            match c {
                b'a' => {
                    val |= 1;
                },
                b'b' => {
                    val |= 1 << 1;
                },
                b'c' => {
                    val |= 1 << 2;
                },
                b'd' => {
                    val |= 1 << 3;
                },
                b'e' => {
                    val |= 1 << 4;
                },
                b'f' => {
                    val |= 1 << 5;
                },
                b'g' => {
                    val |= 1 << 6;
                },
                _ => {},
            }
        }
        val
    }

    #[cfg_attr(target_arch = "x86_64", target_feature(enable = "popcnt"))]
    unsafe fn popcnt(a: u8) -> u8 {
        a.count_ones() as u8
    }

    // Returns the number of bits that are
    // present in the source but are not
    // present in the destination
    fn mismatches(src: u8, dst: u8) -> u8 {
        let mut x = src & dst;
        x ^= src;

        unsafe {
            popcnt(x)
        }
    }

    for line in fil.lines() {
        let line = line.expect("Unable to read line");
        
        if line.trim().is_empty() {
            break;
        }

        let pipe_split: Vec<String> = line.split('|').map(|s| String::from(s.trim())).collect();

        if pipe_split.len() != 2 {
            panic!("Malformed input!");
        }

        let digit_map: Vec<u8> = pipe_split[0].split_whitespace().map(|s| string_to_map(s)).collect();
        let o_vals: Vec<u8> = pipe_split[1].split_whitespace().map(|s| string_to_map(s)).collect();

        digit_maps.push(digit_map);
        output_values.push(o_vals);
    }

    let mut total_o_value: u32 = 0;

    for i in 0..digit_maps.len() {
        let mut digits: [u8; 10] = [255u8; 10];
        
        // Do the digits we know
        digits[1] = {
            unsafe {
                let tmpd: Vec<u8> = digit_maps[i].iter().filter(|&&d| popcnt(d) == 2).map(|d| *d).take(1).collect();
                tmpd[0]
            }            
        };

        digits[4] = {
            unsafe {
                let tmpd: Vec<u8> = digit_maps[i].iter().filter(|&&d| popcnt(d) == 4).map(|d| *d).take(1).collect();
                tmpd[0]
            }
        };

        digits[7] = {
            unsafe {
                let tmpd: Vec<u8> = digit_maps[i].iter().filter(|&&d| popcnt(d) == 3).map(|d| *d).take(1).collect();
                tmpd[0]
            }
        };

        digits[8] = {
            unsafe {
                let tmpd: Vec<u8> = digit_maps[i].iter().filter(|&&d| popcnt(d) == 7).map(|d| *d).take(1).collect();
                tmpd[0]
            }
        };

        digits[3] = {
            let mut r: u8 = 255;
            unsafe {
                let tmpd: Vec<u8> = digit_maps[i].iter().filter(|&&d| popcnt(d) == 5).map(|d| *d).collect();
                
                for x in tmpd {
                    if mismatches(digits[1], x) == 0 {
                        r = x;
                        break;
                    }                
                }
            }
            if r != 255 {
                r
            }
            else {
                panic!("Unable to locate digit 3 in sequence {}", i);
            }
        };

        digits[9] = {
            let mut r: u8 = 255;
            unsafe {
                let tmpd: Vec<u8> = digit_maps[i].iter().filter(|&&d| popcnt(d) == 6).map(|d| *d).collect();
                
                for x in tmpd {
                    if mismatches(digits[3], x) == 0 {
                        r = x;
                        break;
                    }                
                }
            }
            if r != 255 {
                r
            }
            else {
                panic!("Unable to locate digit 9 in sequence {}", i);
            }
        };

        digits[0] = {
            let mut r: u8 = 255;
            unsafe {
                let tmpd: Vec<u8> = digit_maps[i].iter().filter(|&&d| popcnt(d) == 6).map(|d| *d).collect();
                
                for x in tmpd {
                    if mismatches(digits[1], x) == 0 && digits[9] != x {
                        r = x;
                        break;
                    }                
                }
            }
            if r != 255 {
                r
            }
            else {
                panic!("Unable to locate digit 0 in sequence {}", i);
            }
        };

        digits[6] = {
            let mut r: u8 = 255;
            unsafe {
                let tmpd: Vec<u8> = digit_maps[i].iter().filter(|&&d| popcnt(d) == 6).map(|d| *d).collect();
                
                for x in tmpd {
                    if digits[9] == x || digits[0] == x {
                        continue;
                    }
                    else {
                        r = x;
                        break;
                    }      
                }
            }
            if r != 255 {
                r
            }
            else {
                panic!("Unable to locate digit 6 in sequence {}", i);
            }
        };

        digits[2] = {
            let mut r: u8 = 255;
            unsafe {
                let tmpd: Vec<u8> = digit_maps[i].iter().filter(|&&d| popcnt(d) == 5).map(|d| *d).collect();
                
                let diff: u8 = digits[8] ^ digits[9];

                for x in tmpd {
                    if mismatches(diff, x) == 0 && digits[3] != x {
                        r = x;
                        break;
                    }                
                }
            }
            if r != 255 {
                r
            }
            else {
                panic!("Unable to locate digit 2 in sequence {}", i);
            }
        };

        digits[5] = {
            let mut r: u8 = 255;
            unsafe {
                let tmpd: Vec<u8> = digit_maps[i].iter().filter(|&&d| popcnt(d) == 5).map(|d| *d).collect();
                
                for x in tmpd {
                    if digits[2] == x || digits[3] == x {
                        continue;
                    }
                    else {
                        r = x;
                        break;
                    }      
                }
            }
            if r != 255 {
                r
            }
            else {
                panic!("Unable to locate digit 5 in sequence {}", i);
            }
        };

        let mut o_num: u32 = 0;

        for j in 0..output_values[i].len() {
            let mut found: bool = false;

            for k in 0..10 {
                if output_values[i][j] == digits[k] {
                    o_num += k as u32 * u32::pow(10, 3 - j as u32);
                    found = true;
                    break;
                }
            }

            if found == false {
                panic!("Unable to match output digit {} (le) pattern {:b} ({}) in sequence {}: digit map: {:?}, digits: {:?}", j, output_values[i][j], output_values[i][j], i + 1, digit_maps[i], digits);
            }
        }

        total_o_value += o_num;
        println!("Output value (#{}): {:04}", i + 1, o_num);
    }

    println!("Total output value: {}", total_o_value);
}