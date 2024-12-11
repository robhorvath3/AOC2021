use std::mem;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

const ROT: [i32; 6] = [0xE4, // 0-1-2-3
                       0xD8, // 0-2-1-3
                       0xE1, // 1-0-2-3
                       0xC9, // 1-2-0-3
                       0xD2, // 2-0-1-3
                       0xC6];// 2-1-0-3

#[inline(always)]
fn read_xv(pv: &__m256i) -> (i32, i32, i32, i32, i32, i32, i32, i32) {
    unsafe {
        let r: (i32, i32, i32, i32, i32, i32, i32, i32) = mem::transmute(*pv);
        r
    }
}

fn read_xv_as_vec(pv: &__m256i) -> Vec<i32> {
    let mut rv: Vec<i32> = Vec::with_capacity(8);

    unsafe {
        let r: (i32, i32, i32, i32, i32, i32, i32, i32) = mem::transmute(*pv);
        rv.push(r.0);
        rv.push(r.1);
        rv.push(r.2);
        rv.push(r.3);
        rv.push(r.4);
        rv.push(r.5);
        rv.push(r.6);
        rv.push(r.7);
    }
    rv
}

#[inline(always)]
fn negate(vx: &mut __m256i, mask: __m256i) -> &mut __m256i {
    unsafe {
        *vx = _mm256_sign_epi32(*vx, mask);
    }
    vx
}

#[inline(always)]
fn shuffle(vx: &mut __m256i, mask_idx: usize) -> &mut __m256i {
    match mask_idx {
        0 => {
            unsafe {
                *vx = _mm256_shuffle_epi32(*vx, ROT[0]);
            }
        },
        1 => {
            unsafe {
                *vx = _mm256_shuffle_epi32(*vx, ROT[1]);
            }
        },
        2 => {
            unsafe {
                *vx = _mm256_shuffle_epi32(*vx, ROT[2]);
            }
        },
        3 => {
            unsafe {
                *vx = _mm256_shuffle_epi32(*vx, ROT[3]);
            }
        },
        4 => {
            unsafe {
                *vx = _mm256_shuffle_epi32(*vx, ROT[4]);
            }
        },
        5 => {
            unsafe {
                *vx = _mm256_shuffle_epi32(*vx, ROT[5]);
            }
        },
        _ => {
            unsafe {
                *vx = _mm256_setzero_si256();
            }
        },
    }
    vx
}

fn main() {
    let f = File::open("./input.txt").expect("Unable to open input file");
    let f = BufReader::new(f);
    
    let mut sign: Vec<__m256i> = Vec::with_capacity(8);
    unsafe {
        sign.push(_mm256_setr_epi32(1, 1, 1, 99, 1, 1, 1, 33));
        sign.push(_mm256_setr_epi32(1, 1, -1, 99, 1, 1, -1, 33));
        sign.push(_mm256_setr_epi32(1, -1, 1, 99, 1, -1, 1, 33));
        sign.push(_mm256_setr_epi32(1, -1, -1, 99, 1, -1, -1, 33));
        sign.push(_mm256_setr_epi32(-1, 1, 1, 99, -1, 1, 1, 33));
        sign.push(_mm256_setr_epi32(-1, 1, -1, 99, -1, 1, -1, 33));
        sign.push(_mm256_setr_epi32(-1, -1, 1, 99, -1, -1, 1, 33));
        sign.push(_mm256_setr_epi32(-1, -1, -1, 99, -1, -1, -1, 33));
    }

    let rots: [[usize; 2]; 24] = [[0,0], [1,2], [0,3], [1,1],
                                  [0,6], [1,4], [0,5], [1,7],
                                  [3,0], [2,2], [3,3], [2,1],
                                  [3,6], [2,4], [3,5], [2,7],
                                  [4,0], [5,2], [4,3], [5,1],
                                  [4,6], [5,4], [4,5], [5,7]];

    
    let mut status: Vec<bool> = Vec::with_capacity(64);
    let mut run: Vec<bool> = Vec::with_capacity(64);

    let mut scanners: Vec<Vec<__m256i>> = Vec::with_capacity(64);
    let mut scanner_pos: Vec<__m256i> = Vec::with_capacity(64);

    {
        let mut cs: usize = 0;
        let mut pc: usize = 0;
        let mut tpv: Vec<__m256i>;
        let mut txv: __m256i;
        let mut ibuf: [i32; 3] = [0, 0, 0];
        let mut bbuf: bool = false;

        for line in f.lines() {
            let line = line.expect("Unable to read line");
            
            if line.trim().is_empty() {
                continue;            
            }

            if line.get(0..=1).unwrap() == "--" {
                if bbuf {
                    unsafe {
                        txv = _mm256_setr_epi32(ibuf[0], ibuf[1], ibuf[2], 33, 1500000, 1500000, 150000, 99);                        
                    }
                    scanners[cs].push(txv);
                    bbuf = false;
                }
                
                tpv = Vec::with_capacity(16);
                cs = scanners.len();
                pc = 0;
                scanners.push(tpv);
                status.push(false);
                run.push(false);
                unsafe {
                    scanner_pos.push(_mm256_setr_epi32(0, 0, 0, 0, 0, 0, 0, 0));
                }
                //println!("Beginning scanner {}", cs);
            }
            else {
                let ts: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
                //println!("Reading input line for cs {}, ts: {:?}", cs, ts);

                if pc % 2 == 0 && cs != 0 {
                    ibuf = [ts[0], ts[1], ts[2]];
                    bbuf = true;
                    //println!("Buffering for pair {} ibuf: {:?}", pc, ibuf);
                    pc += 1;
                }
                else {
                    if cs > 0 {
                        unsafe {
                            txv = _mm256_setr_epi32(ibuf[0], ibuf[1], ibuf[2], 33, ts[0], ts[1], ts[2], 99);                        
                        }
                    }
                    else {
                        unsafe {
                            txv = _mm256_setr_epi32(ts[0], ts[1], ts[2], 11, ts[0], ts[1], ts[2], 22);
                        }
                    }
                    //println!("Got AVX2 vector for pair {} txv: {:?}", pc, txv);
                    scanners[cs].push(txv);
                    bbuf = false;
                    pc += 1;
                }         
            }
        }

        if bbuf {
            unsafe {
                txv = _mm256_setr_epi32(ibuf[0], ibuf[1], ibuf[2], 33, 1500000, 1500000, 150000, 99);                        
            }
            scanners[cs].push(txv);
        }
    }

    status[0] = true;

    let mut total_matches: usize = 0;
    let mut ms: Vec<__m256i> = Vec::with_capacity(256);

    for i in 0..scanners[0].len() {
        ms.push(scanners[0][i]);
    }

    while total_matches < scanners.len() - 1 {
        let mut current_score: usize = 0;

        for i in 0..scanners.len() {
            if !status[i] || run[i] {
                continue;
            }

            let score = do_matches_against(i, &mut ms, &mut scanners, &mut scanner_pos, &rots, &sign, &mut status);
            total_matches += score;
            current_score += score;
            println!("Total matches ({}): {}", i, total_matches);
            run[i] = true;
        }

        if current_score == 0 {
            panic!("Unable to match all scanners.");
        }
    }

    let mut final_map: HashMap<String, bool> = HashMap::with_capacity(256);

    for i in 0..ms.len() {
        let t = read_xv(&ms[i]);
        let s1 = format!("{},{},{}", t.0, t.1, t.2);

        if !final_map.contains_key(&s1) {
            println!("{}", s1);
            final_map.insert(s1, true);
        }
    }

    println!("All scanners matched: beacon count: {}", final_map.len());

    let mut largest_dist: i32 = 0;

    // manhattan distances
    for i in 0..scanner_pos.len() {
        for j in 0..scanner_pos.len() {
            if j == i {
                continue;
            }

            let v: __m256i;
            unsafe {
                v = _mm256_abs_epi32(_mm256_sub_epi32(scanner_pos[j], scanner_pos[i]));
            }

            let vc = read_xv(&v);
            let dist = vc.0 + vc.1 + vc.2;

            if dist > largest_dist {
                largest_dist = dist;
            }
        }
    }

    println!("Largest Manhattan Distance Between Scanners: {}", largest_dist);

    fn do_matches_against(idx: usize, ms: &mut Vec<__m256i>, scanners: &mut Vec<Vec<__m256i>>, scanner_pos: &mut Vec<__m256i>, rots: &[[usize; 2]; 24], sign: &Vec<__m256i>, status: &mut Vec<bool>) -> usize {
        let mut scanner_matches: usize = 0;

        let cmp_mask: __m256i;
        unsafe {
            cmp_mask = _mm256_setr_epi32(0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80);
        }
        
        let mut src_cmp: Vec<__m256i> = Vec::with_capacity(32);

        if idx == 0 {
            src_cmp = scanners[idx].clone();
        }
        else {
            for i in 0..scanners[idx].len() {
                let t = read_xv(&scanners[idx][i]);
                
                unsafe {
                    src_cmp.push(_mm256_setr_epi32(t.0, t.1, t.2, 11, t.0, t.1, t.2, 22));
                    src_cmp.push(_mm256_setr_epi32(t.4, t.5, t.6, 11, t.4, t.5, t.6, 22));
                }
            }
        }

        // Compare scanner with all other unmatched scanners
        'smatch: for si in 1..scanners.len() {

            if status[si] == true || si == idx {
                continue;
            }
            
            // for each given rotation
            for ri in 0..24 {
                
                // get a local copy
                let mut scanner: Vec<__m256i> = scanners[si].clone();

                // rotate the points
                for pi in 0..scanner.len() {
                    shuffle(&mut scanner[pi], rots[ri][0]);
                    negate(&mut scanner[pi], sign[rots[ri][1]]);
                    //println!("rot {}: {:?}", ri, read_xv(&scanner[pi]));
                }
                                
                // go through all source scanner points
                for offi in 0..src_cmp.len() {
                    let tv = read_xv(&src_cmp[offi]);
                    if tv.4 > 1000000 || tv.4 < -1000000 ||
                        tv.5 > 1000000 || tv.5 < -1000000 ||
                        tv.6 > 1000000 || tv.6 < -1000000 {
                            continue;
                    }

                    for offj in 0..scanner.len() * 2 {
                        // make a copy of this rotated matrix
                        let mut lscanner = scanner.clone();

                        // find an offset for each point and apply
                        // until we find 12 matches, or rotate again
                        unsafe {
                            let off_raw = _mm256_sub_epi32(lscanner[offj/2], src_cmp[offi]);
                            let off_un = read_xv(&off_raw);
                            let offset: __m256i;

                            if offj % 2 == 0 {
                                offset = _mm256_setr_epi32(off_un.0, off_un.1, off_un.2, 0, off_un.0, off_un.1, off_un.2, 0);
                            }
                            else {
                                let tv = read_xv(&lscanner[offj/2]);
                                if tv.4 > 1000000 || tv.4 < -1000000 ||
                                    tv.5 > 1000000 || tv.5 < -1000000 ||
                                    tv.6 > 1000000 || tv.6 < -1000000 {
                                        continue;
                                }
                                
                                offset = _mm256_setr_epi32(off_un.4, off_un.5, off_un.6, 0, off_un.4, off_un.5, off_un.6, 0);
                            }
                            //println!("Current offset: {:?}", read_xv(&offset));

                            for zi in 0..lscanner.len() {
                                lscanner[zi] = _mm256_sub_epi32(lscanner[zi], offset);
                                //println!("offset applied: {:?}", read_xv(&lscanner[zi]));
                            }
                                                        
                            let mut beacon_matches: usize = 0;
                            let mut matched_idx: HashMap<usize, bool> = HashMap::with_capacity(32);

                            for ii in 0..src_cmp.len() {
                                
                                let tv = read_xv(&src_cmp[ii]);
                                if tv.4 > 1000000 || tv.4 < -1000000 ||
                                    tv.5 > 1000000 || tv.5 < -1000000 ||
                                    tv.6 > 1000000 || tv.6 < -1000000 {
                                        continue;
                                }
                                
                                for ij in 0..lscanner.len() {
                                    //println!("Comparing s0p{} with s{}p{} at rot {} with parent point at s0p{}, local point at s{}p{}",
                                        //ii, si, ij, ri, offi, si, offj);

                                    let cmpv = _mm256_and_si256(_mm256_cmpeq_epi32(src_cmp[ii], lscanner[ij]), cmp_mask);
                                    let cmp_un = read_xv_as_vec(&cmpv);
                                    let mut match_score: u8 = 0;

                                    for zzz in 0..cmp_un.len() {
                                        match_score |= cmp_un[zzz] as u8;
                                    }

                                    if match_score & 0x07 == 0x07 {
                                        beacon_matches += 1;
                                        matched_idx.insert(ij * 2, true);
                                        //println!("Matched beacon @ ri {} s0p{} with s{}p{}, beacon matches: {}, raw match count: {}, offi: {}, offj: {}", ri, ii, si, ij, beacon_matches, match_score, offi, offj);                                       
                                    }
                                    else if match_score & 0x70 == 0x70 {
                                        let tv = read_xv(&lscanner[ij]);
                                        if tv.4 > 1000000 || tv.4 < -1000000 ||
                                           tv.5 > 1000000 || tv.5 < -1000000 ||
                                           tv.6 > 1000000 || tv.6 < -1000000 {
                                               continue;
                                        }

                                        beacon_matches += 1;
                                        matched_idx.insert((ij * 2)+1, true);
                                        //println!("Matched beacon @ ri {} s0p{} with s{}p{}, beacon matches: {}, raw match count: {}, offi: {}, offj: {}", ri, ii, si, ij, beacon_matches, match_score, offi, offj);
                                    }                                                                        
                                }
                            }

                            if beacon_matches >= 12 {
                                scanners[si] = lscanner.clone();
                                status[si] = true;
                                
                                let tv = read_xv(&offset);
                                scanner_pos[si] = _mm256_setr_epi32(tv.0, tv.1, tv.2, 0, 0, 0, 0, 0);

                                scanner_matches += 1;

                                for i in 0..scanners[si].len() * 2 {
                                    if !matched_idx.contains_key(&i) {
                                        let tv = read_xv(&scanners[si][i/2]);

                                        if i % 2 == 0 {
                                            ms.push(_mm256_setr_epi32(tv.0, tv.1, tv.2, 11, tv.0, tv.1, tv.2, 22));
                                        }
                                        else {
                                            if tv.4 > 1000000 || tv.4 < -1000000 ||
                                               tv.5 > 1000000 || tv.5 < -1000000 ||
                                               tv.6 > 1000000 || tv.6 < -1000000 {
                                                continue;
                                            }
                                            ms.push(_mm256_setr_epi32(tv.4, tv.5, tv.6, 11, tv.4, tv.5, tv.6, 22));
                                        }
                                    }
                                }
                                continue 'smatch;
                            }
                        }
                    }                
                }
            }
        }
        scanner_matches
    }
}