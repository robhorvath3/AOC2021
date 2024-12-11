use std::collections::HashMap;

#[derive(Debug)]
struct ValidPair {
    v: i64,
    steps: i64,
}

#[inline(always)]
fn sum_of_ints(n: i64) -> i64 {
    let mut x = n;

    if n == 0 {
        0
    }
    else if n < 0 {
        x = x.abs();
        -1 * ((x * (x+1)) / 2)
    }
    else {
        (x * (x+1)) / 2
    }
}

fn find_v_y_between(lbound: i64, ubound: i64) -> Vec<ValidPair> {
    let mut v: Vec<ValidPair> = Vec::new();
    let minb = i64::min(lbound, ubound);
    let maxb = i64::max(lbound, ubound);

    //println!("debug vy: lbound: {}, ubound: {}", minb, maxb);

    for cv in i64::min(minb, 0)..=i64::max(minb.abs(), maxb.abs()) {
        let mut n: i64 = 1;

        loop {
            let pos = y_pos(cv, n);
            
            if pos >= minb && pos <= maxb {
                v.push(ValidPair { v: cv, steps: n });                
            }
            else if pos < minb {
                break;
            }
            n += 1;
        }
    }
    v
}

fn find_v_x_between(lbound: i64, ubound: i64) -> Vec<ValidPair> {
    let mut v: Vec<ValidPair> = Vec::new();
    let minb = i64::min(lbound, ubound);
    let maxb = i64::max(lbound, ubound);

    //println!("debug vx: lbound: {}, ubound: {}", minb, maxb);

    for cv in i64::min(minb, 0)..=i64::max(minb.abs(), maxb.abs()) {
        let mut n: i64 = 1;

        loop {
            let pos = x_pos(cv, n);
            let xv = vx_at_step(cv, n);

            //println!("Looping cv: {}, n: {}, pos: {}, xv: {}", cv, n, pos, xv);

            if xv == 0 && pos >= minb && pos <= maxb {
                v.push(ValidPair { v: cv, steps: n });
                break;
            }
            else if xv == 0 && (pos <= minb || pos >= maxb) {
                break;
            }
            else if pos >= minb && pos <= maxb {
                v.push(ValidPair { v: cv, steps: n });                
            }
            else if pos > maxb {
                break;
            }
            else if cv == n {
                break;
            }

            n += 1;
        }
    }
    v
}

#[inline(always)]
fn vx_at_step(vi: i64, i: i64) -> i64 {
    i64::max(0, vi - i)
}

fn x_pos(vi: i64, i: i64) -> i64 {
    if i < vi {
        sum_of_ints(vi) - sum_of_ints(vi - i % vi)
    }
    else {
        sum_of_ints(vi)
    }
}

fn y_pos(vi: i64, i: i64) -> i64 {
    if i == 0 {
        return 0;
    }

    if vi < 0 {
        sum_of_ints(vi - i + 1) - sum_of_ints(vi + 1)
    }
    else if vi == 0 {
        -1 * sum_of_ints(i - 1)
    }
    else {
        if i <= vi {
            sum_of_ints(vi) - sum_of_ints(vi - i)
        }
        else if i == vi + 1 {
            sum_of_ints(vi)
        }
        else if i > vi + 1 && i <= (2 * vi) + 1 {
            sum_of_ints(vi) - sum_of_ints(i - vi - 1)
        }
        else {
            y_pos(-1 * (vi + 1), i - ((2 * vi) + 1))
        }
    }
}

fn main() {
    let x0 = 179;
    let x1 = 201;
    
    let y0: i64 = -109;
    let y1: i64 = -63;

    let v_x: Vec<ValidPair> = find_v_x_between(x0, x1);
    //println!("Valid x velocities2 for landing between x0 {} and x1 {}: {:?}", x0, x1, v_x);

    let v_y: Vec<ValidPair> = find_v_y_between(y0, y1);
    //println!("Valid y velocities for landing between y0 {} and y1 {}: {:?}", y0, y1, v_y);

    let mut pairs: HashMap<String, bool> = HashMap::new();

    for x in &v_x {
        for y in &v_y {
            if x.steps == y.steps || (x.v == x.steps && y.steps >= x.steps) {                
                let k: String = format!("{},{}", x.v, y.v);
                if !pairs.contains_key(&k) {
                    pairs.insert(k, true);
                    //println!("({},{}) step {}", x.v, y.v, y.steps);
                }
            }
        }
    }

    println!("Found {} total pairs", pairs.len());
}