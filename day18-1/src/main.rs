use std::fs::File;
use std::io::{BufRead, BufReader};

const LBRACK: u32 = u32::MAX;
const RBRACK: u32 = u32::MAX-1;
const COMMA: u32 = u32::MAX-2;
const DESTROY: u32 = u32::MAX-3;

fn add_encoded(v1: &mut Vec<u32>, v2: &Vec<u32>) {
    let mut vr: Vec<u32> = Vec::with_capacity(64);

    vr.push(LBRACK);

    for i in 0..v1.len() {
        vr.push(v1[i]);
    }

    vr.push(COMMA);

    for i in 0..v2.len() {
        vr.push(v2[i]);
    }

    vr.push(RBRACK);

    v1.clear();

    for i in 0..vr.len() {
        v1.push(vr[i]);
    }
}

fn encode(s: &String) -> Vec<u32> {
    let mut vr: Vec<u32> = Vec::with_capacity(64);
    let chars: Vec<char> = s.chars().collect();

    let mut skip: bool = false;
    let clen: usize = chars.len();

    for i in 0..clen {
        if skip {
            skip = false;
            continue
        }

        match chars[i] {
            '[' => {
                vr.push(LBRACK);
            },
            ']' => {
                vr.push(RBRACK);
            },
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
                if i != clen - 1 && chars[i+1].is_digit(10) {
                    skip = true;
                    vr.push(((chars[i].to_digit(10).unwrap() * 10) + chars[i+1].to_digit(10).unwrap()) as u32);
                }
                else {
                    vr.push(chars[i].to_digit(10).unwrap() as u32);
                }
            },
            ',' => {
                vr.push(COMMA);
            },
            _ => {},
        }
    }
    vr
}

fn magnitude(v: &mut Vec<u32>) -> &mut Vec<u32> {
    let mut rv: Vec<u32> = Vec::with_capacity(64);
    let mut found_pair: bool = false;

    loop {
        let mut i: usize = 0;
        
        while i < v.len()-4 {
            if v[i] == LBRACK && v[i+1] < DESTROY && v[i+2] == COMMA && v[i+3] < DESTROY && v[i+4] == RBRACK {
                found_pair = true;
                
                /*
                print!("Reducing Magnitude1: ");
                printv(&v);
                print!("\n");
                */

                v[i+4] = (3 * v[i+1]) + (2 * v[i+3]);
                v[i] = DESTROY;
                v[i+1] = DESTROY;
                v[i+2] = DESTROY;
                v[i+3] = DESTROY;
                i += 4;

                /*
                print!("Reducing Magnitude2: ");
                printv(&v);
                print!("\n");
                */
            }
            i += 1;
        }

        if found_pair {
            for i in 0..v.len() {
                if v[i] != DESTROY {
                    rv.push(v[i]);
                }
            }

            v.clear();
            
            for i in 0..rv.len() {
                v.push(rv[i]);
            }

            rv.clear();

            /*
            print!("Reducing Magnitude3: ");
            printv(&v);
            print!("\n");
            */

            found_pair = false;

            if v.len() == 1 {
                break;
            }
        }
        else {
            break;
        }
    }

    v
}

fn reduce(encoded: &mut Vec<u32>) -> &mut Vec<u32> {
    let mut rv: Vec<u32> = Vec::with_capacity(64);
    
    let mut exploded: bool;
    let mut split: bool;
    let mut vlen: usize;

    let mut pos: usize;

    loop {
        exploded = false;
        split = false;
        
        // Check for 'explode' first
        vlen = encoded.len();
        pos = 0;
        let mut depth: usize = 0;

        loop {
            match encoded[pos] {
                LBRACK => {
                    depth += 1;

                    if depth == 5 {
                        let lb_idx = pos;
                        let lnum_idx = pos + 1;
                        let comma_idx = pos + 2;
                        let rnum_idx = pos + 3;
                        let rb_idx = pos + 4;

                        // add left number to next left number (if any)
                        for i in (1..=lb_idx-1).rev() {
                            //println!("Checking add left number @ pos {}", i);
                            if encoded[i] < DESTROY {
                                encoded[i] += encoded[lnum_idx];
                                break;
                            }
                        }

                        // add right number to next right number (if any)
                        for i in rb_idx+1..vlen {
                            //println!("Checking add right number @ pos {}", i);
                            if encoded[i] < DESTROY {
                                encoded[i] += encoded[rnum_idx];
                                break;
                            }
                        }

                        encoded[lb_idx] = DESTROY;
                        encoded[lnum_idx] = DESTROY;
                        encoded[comma_idx] = DESTROY;
                        encoded[rnum_idx] = DESTROY;
                        encoded[rb_idx] = 0;

                        // re-write encoded to rv
                        for i in 0..vlen {
                            if encoded[i] != DESTROY {
                                rv.push(encoded[i]);
                            }
                        }

                        // clear encoded and set it equal to rv
                        encoded.clear();
                        
                        for i in 0..rv.len() {
                            encoded.push(rv[i]);
                        }

                        // clear rv
                        rv.clear();

                        exploded = true;

                        break;
                    }
                },
                RBRACK => {
                    depth -= 1;
                },
                _ => {},
            }

            pos += 1;

            if pos >= vlen {
                break;
            }
        }

        // Check for 'split' after iff no explodes
        if !exploded {
            vlen = encoded.len();

            for i in 0..vlen {
                if encoded[i] < DESTROY && encoded[i] >= 10 {
                    // push everything up to this point
                    for j in 0..i {
                        rv.push(encoded[j]);
                    }
                    
                    // push a new left bracket
                    rv.push(LBRACK);

                    // new left value is encoded[i] / 2
                    rv.push(encoded[i] / 2);

                    // push a comma
                    rv.push(COMMA);

                    // new right value is (encoded[i] + 1) / 2
                    rv.push((encoded[i] + 1) / 2);

                    // push a new right bracket
                    rv.push(RBRACK);

                    // push everything after this point
                    for j in i+1..vlen {
                        rv.push(encoded[j]);
                    }

                    // clear our encoded vector
                    encoded.clear();

                    // push everything in rv back to encoded
                    for j in 0..rv.len() {
                        encoded.push(rv[j]);
                    }

                    // clear rv
                    rv.clear();

                    split = true;

                    break;
                }
            }
        }
        if !exploded && !split {
            break;
        }
    }
    encoded
}

fn printv(v: &Vec<u32>) {
    for i in 0..v.len() {
        match v[i] {
            DESTROY => {
                print!("<D>");
            },
            LBRACK => {
                print!("[");
            },
            RBRACK => {
                print!("]");
            },
            COMMA => {
                print!(",");
            },
            _ => {
                print!("{}", v[i]);
            },
        }
    }
}

fn main() {
    let f = File::open("./input.txt").expect("Unable to open input file");
    let f = BufReader::new(f);
    
    let mut nums: Vec<Vec<u32>> = Vec::with_capacity(100);
    
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        
        if line.trim().is_empty() {
            break;            
        }

        let enc: Vec<u32> = encode(&line);
        nums.push(enc);
    }

    let mut v: Vec<u32> = nums[0].clone();
    print!("Starting v: ");
    printv(&v);
    print!("\n");

    for i in 1..nums.len() {
        print!("Adding v + nums[{}]: ", i);
        printv(&nums[i]);
        print!("\n");
        add_encoded(&mut v, &nums[i]);
        print!("Result of v + nums[{}]: ", i);
        printv(&v);
        print!("\n");
        reduce(&mut v);
        print!("Reducing v: ");
        printv(&v);
        print!("\n");
    }
    
    //println!("{:?}", nums);
    //println!("Encoded v: {:?}", v);
    print!("v: ");
    printv(&v);
    print!("\n");

    println!("The final magnitude is {:?}", magnitude(&mut v));

    //println!("Counted {} points remaining", points_remaining.len());
}