use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("./input.txt").expect("Unable to open input file");
    let f = BufReader::new(f);

    let mut x_offset: usize = 32;
    let mut y_offset: usize = 10;

    let mut mask: [u64; 8] = [0u64; 8];
    let mut input: Vec<[u64; 3]> = Vec::with_capacity(128);

    for _ in 0..y_offset {
        input.push([0u64; 3]);
    }

    {
        let mut blank: bool = false;

        for line in f.lines() {
            let line = line.expect("Unable to read line");
            
            if line.trim().is_empty() {
                if !blank {
                    blank = true;
                    continue;
                }
                else {
                    break;
                }
            }

            if !blank {
                let mut cidx: usize = 0;
                for c in line.chars() {
                    match c {
                        '#' => {
                            mask[cidx / 64] |= 1 << (cidx % 64);
                        },
                        _ => {}
                    }
                    cidx += 1;
                }
            }
            else {
                let mut cidx: usize = x_offset;
                let mut vals: [u64; 3] = [0u64; 3];

                for c in line.chars() {
                    match c {
                        '#' => {
                            vals[cidx / 64] |= 1 << (cidx % 64);
                        },
                        _ => {}
                    }
                    cidx += 1;
                }

                input.push(vals);
            }
        }
    }

    // push 10 more onto the end
    for _ in 0..y_offset {
        input.push([0u64; 3]);
    }

    let mut output1: Vec<[u64; 3]> = Vec::with_capacity(128);
    let mut output2: Vec<[u64; 3]> = Vec::with_capacity(128);
    for _i in 0..120 {
        output1.push([u64::MAX; 3]);
        output2.push([0u64; 3]);
    }

    #[inline(always)]
    // read_num's x and y are the lower right corner's x & y of the
    // 9 cell box to get the mask index; this actually controls the
    // center pixel, or (x-1,y-1)
    fn read_num(input: &Vec<[u64; 3]>, x: usize, y: usize) -> usize {
        let mut rval: usize = 0;
        for iy in (y-2)..=y {
            for ix in (x-2)..=x {
                if (input[iy][ix/64] & (1 << (ix % 64))) == 1 << (ix % 64) {
                    rval |= 1 << (((y-iy) * 3) + (x-ix));
                } 
            }
        }
        rval
    }

    #[inline(always)]
    fn read_mask(mask: &[u64; 8], num: usize) -> bool {
        if (mask[num / 64] & (1 << (num as u64 % 64))) == 1 << (num as u64 % 64) {
            true
        }
        else {
            false
        }
    }

    #[inline(always)]
    fn write_canvas(canvas: &mut Vec<[u64; 3]>, x: usize, y: usize, is_lit: bool) {
        if is_lit {
            canvas[y][x / 64] |= 1 << (x as u64 % 64);
        }
        else {
            canvas[y][x / 64] &= u64::MAX ^ (1 << (x as u64 % 64));
        }
    }

    // write the input to the output at the specified offsets
    for iy in (y_offset-1)..(y_offset+101) {
        for ix in (x_offset-1)..(x_offset+101) {
            let num = read_num(&input, ix+1, iy+1);
            let maskval = read_mask(&mask, num);
            //println!("writing pixel at ({},{}), index {}", ix, iy, num);
            write_canvas(&mut output1, ix, iy, maskval);
        }
    }

    // toggle edge bits on
    /*
    for iy in (y_offset-3)..=(y_offset+103) {
        if iy == y_offset - 3 || iy == y_offset + 103 ||
           iy == y_offset - 2 || iy == y_offset + 102 {
            for ix in (x_offset-3)..=(x_offset+103) {
                write_canvas(&mut output1, ix, iy, true);
            }
        }
        else {
            write_canvas(&mut output1, x_offset-2, iy, true);
            write_canvas(&mut output1, x_offset-3, iy, true);
            write_canvas(&mut output1, x_offset+102, iy, true);
            write_canvas(&mut output1, x_offset+103, iy, true);
        }
    }
    */

    // do it again
    for iy in (y_offset-2)..(y_offset+102) {
        for ix in (x_offset-2)..(x_offset+102) {
            let num = read_num(&output1, ix+1, iy+1);
            let maskval = read_mask(&mask, num);
            //println!("writing pixel at ({},{}), index {}", ix, iy, num);
            write_canvas(&mut output2, ix, iy, maskval);
        }
    }

    /*
    // toggle edge bits off
    for iy in (y_offset-3)..=(y_offset+103) {
        if iy == y_offset - 3 || iy == y_offset + 103 {
            for ix in (x_offset-3)..=(x_offset+103) {
                write_canvas(&mut output2, ix, iy, false);
            }
        }
        else {
            write_canvas(&mut output2, x_offset-3, iy, false);
            write_canvas(&mut output2, x_offset+103, iy, false);
        }
    }
    */

    #[cfg_attr(target_arch = "x86_64", target_feature(enable = "popcnt"))]
    unsafe fn popcnt(a: u64) -> u64 {
        a.count_ones() as u64
    }

    // count how many pixels are illuminated
    let mut pixel_count: usize = 0;
    
    for i in 0..output2.len() {
        for j in 0..3 {
            unsafe {
                let pc = popcnt(output2[i][j]) as usize;
                println!("({},{}) == {} and has {} pixels illuminated", j, i, output2[i][j], pc);
                pixel_count += pc;
            }
        }
    }

    println!("After 2 rounds, there are {} pixels illuminated", pixel_count);
}