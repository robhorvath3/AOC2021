#[derive(Debug)]
struct ValidPair {
    v: i32,
    steps: i32,
}

#[inline(always)]
fn sum_of_ints(n: i32) -> i32 {
    let mut x = n;

    if n < 0 {
        x = x.abs();
        -1 * ((x * (x+1)) / 2)
    }
    else {
        (x * (x+1)) / 2
    }
}

#[inline(always)]
fn sum_of_ints_between(n1: i32, n2: i32) -> i32 {
    let min_n = i32::min(n1, n2);
    let max_n = i32::max(n1, n2);

    if min_n >= 0 && max_n >= 0 {
        sum_of_ints(max_n) - sum_of_ints(min_n-1)
    }
    else if min_n < 0 && max_n >= 0 {
        sum_of_ints(min_n.abs()) + sum_of_ints(max_n)
    }
    else {
        sum_of_ints(max_n.abs()) - sum_of_ints(min_n.abs())
    }
}

fn find_v_x_between(lbound: i32, ubound: i32) -> Vec<ValidPair> {
    let mut v: Vec<ValidPair> = Vec::new();
    let mut n = 1;

    println!("debug: lbound: {}, ubound: {}", lbound, ubound);

    loop {
        let sum = sum_of_ints(n);

        if sum >= lbound && sum <= ubound {
            v.push(ValidPair { v: n, steps: n });
        }
        else if sum >= ubound && (n + n - 1) <= ubound {
            println!("debug: n: {}, sum: {}", n, sum);
            let mut p = 0;
            let mut i = n;

            while i > 0 {
                p += i;
                i -= 1;

                if p >= lbound && p <= ubound {
                    println!("debug: pushing n: {}, p: {}, i: {}", n, p, i);
                    v.push(ValidPair { v: n, steps: i });
                    //break;
                }
            }
        }
        else if sum >= ubound && n >= lbound && n <= ubound {
            v.push(ValidPair { v: n, steps: 1 } );
        }
        else if sum >= ubound && n > ubound {
            break;
        }

        n += 1;
    }
    v
}

fn y_pos(vi: i32, i: i32) -> i32 {
    if i == 0 {
        0
    }
    else if vi < 0 {
        //println!("((sum_of_ints(vi.abs() + i)): {} - sum_of_ints(vi.abs()): {})", sum_of_ints(vi.abs() + i), sum_of_ints(vi.abs()));
        -1 * (sum_of_ints(vi.abs() + i - 1) - sum_of_ints(vi.abs() - 1))
    }
    else if i < vi {
        sum_of_ints(vi) - sum_of_ints(vi - i % vi)
    }
    else if i == vi {
        sum_of_ints(vi)
    }
    else if i > vi && i < 2 * vi {
        sum_of_ints(vi) - sum_of_ints(i % vi)
    }
    else if i == 2 * vi {
        0
    }
    else if i > 2 * vi {
        -1 * (sum_of_ints(vi + (i - (2 * vi))) - sum_of_ints(vi))
    }
    else {
        9999
    }
}

fn find_v_y_at_n_steps_between(n: i32, lbound: i32, ubound: i32) -> Vec<ValidPair> {
    let mut v: Vec<ValidPair> = Vec::new();
    let minb = i32::min(lbound, ubound);
    let maxb = i32::max(lbound, ubound);

    let v0 = {
        if minb < 0 {
            minb
        }
        else {
            0
        }
    };

    let v1 = {
        if maxb < 0 {
            maxb.abs()
        }
        else {
            maxb
        }
    };

    for cv in v0..v1 {
        let pos = y_pos(cv, n);

        if pos >= minb && pos <= maxb {
            v.push(ValidPair { v: cv, steps: n });
            //break;
        }                            
    }
    v
}

fn main() {
    let x0 = 156;
    let x1 = 202;
    
    let y0: i32 = -110;
    let y1: i32 = -69;

    let v_x: Vec<ValidPair> = find_v_x_between(x0, x1);
    println!("Valid x velocities for landing between x0 {} and x1 {}: {:?}", x0, x1, v_x);

    for step in 0..250 {
        println!("y pos(109, {}): {}", step, y_pos(109, step));
    }

    /*
    for x in &v_x {
        let v_y: Vec<ValidPair> = find_v_y_at_n_steps_between(x.steps, y0, y1);
        println!("Found valid y velocities for landing between y0 {} and y1 {}: {:?}", y0, y1, v_y);
    }
    */
}