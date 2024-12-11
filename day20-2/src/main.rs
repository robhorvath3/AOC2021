use std::fs::File;
use std::io::{BufRead, BufReader};

const X_OFFSET: usize = 60;
const Y_OFFSET: usize = 60;

fn main() {
    let f = File::open("./input.txt").expect("Unable to open input file");
    let f = BufReader::new(f);

    let mut mask: [u64; 8] = [0u64; 8];
    let mut input: Vec<[u64; 4]> = Vec::with_capacity(256);

    for _ in 0..Y_OFFSET {
        input.push([0u64; 4]);
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
                let mut cidx: usize = X_OFFSET;
                let mut vals: [u64; 4] = [0u64; 4];

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

    // push 'Y_OFFSET' more onto the end
    for _ in 0..Y_OFFSET {
        input.push([0u64; 4]);
    }

    #[inline(always)]
    fn clear_canvas(canvas: &mut Vec<[u64; 4]>, level: usize) {
        canvas.clear();

        if level % 2 == 0 {
            for _i in 0..(2*Y_OFFSET) + 100 {
                canvas.push([0u64; 4]);
            }
        }
        else {
            for _i in 0..(2*Y_OFFSET) + 100 {
                canvas.push([u64::MAX; 4]);
            }
        }
    
    }
    
    #[inline(always)]
    // read_num's x and y are the lower right corner's x & y of the
    // 9 cell box to get the mask index; this actually controls the
    // center pixel, or (x-1,y-1)
    fn read_num(input: &Vec<[u64; 4]>, x: usize, y: usize) -> usize {
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
    fn write_canvas(canvas: &mut Vec<[u64; 4]>, x: usize, y: usize, is_lit: bool) {
        if is_lit {
            canvas[y][x / 64] |= 1 << (x as u64 % 64);
        }
        else {
            canvas[y][x / 64] &= u64::MAX ^ (1 << (x as u64 % 64));
        }
    }

    // write the input to the output at the specified offsets
    fn apply_filter(level: usize, input: &Vec<[u64; 4]>, output: &mut Vec<[u64; 4]>, mask: &[u64; 8]) {
        clear_canvas(output, level);
        for iy in (Y_OFFSET-level)..(Y_OFFSET+level+100) {
            for ix in (X_OFFSET-level)..(X_OFFSET+level+100) {
                let num = read_num(&input, ix+1, iy+1);
                let maskval = read_mask(&mask, num);
                //println!("writing pixel at ({},{}), index {}", ix, iy, num);
                write_canvas(output, ix, iy, maskval);
            }
        }
    }

    let mut output_even: Vec<[u64; 4]> = Vec::with_capacity(256);
    let mut output_odd: Vec<[u64; 4]> = Vec::with_capacity(256);

    apply_filter(1, &input, &mut output_odd, &mask);

    for i in 2..=50 {
        if i % 2 == 0 {
            apply_filter(i, &output_odd, &mut output_even, &mask);
        }
        else {
            apply_filter(i, &output_even, &mut output_odd, &mask);
        }
    }

    #[cfg_attr(target_arch = "x86_64", target_feature(enable = "popcnt"))]
    unsafe fn popcnt(a: u64) -> u64 {
        a.count_ones() as u64
    }

    // count how many pixels are illuminated
    let mut pixel_count: usize = 0;
    
    for i in 0..output_even.len() {
        for j in 0..4 {
            unsafe {
                let pc = popcnt(output_even[i][j]) as usize;
                //println!("({},{}) == {} and has {} pixels illuminated", j, i, output_even[i][j], pc);
                pixel_count += pc;
            }
        }
    }

    println!("After 50 rounds, there are {} pixels illuminated", pixel_count);
}