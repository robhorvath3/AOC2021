use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;

const X_AXIS: usize = 0;
const Y_AXIS: usize = 1;
const Z_AXIS: usize = 2;

#[derive(Debug, Clone, Copy)]
struct Cuboid {
    is_on: bool,
    minval: [i32; 3],
    maxval: [i32; 3],
}

impl Cuboid {
    fn new() -> Cuboid {
        Cuboid {
            is_on: false,
            minval: [0i32; 3],
            maxval: [i32::MAX; 3],
        }
    }

    fn cubes(&self) -> i64 {
        if self.is_on {
            ((self.maxval[X_AXIS] - self.minval[X_AXIS]) + 1) as i64 * ((self.maxval[Y_AXIS] - self.minval[Y_AXIS]) + 1) as i64 * ((self.maxval[Z_AXIS] - self.minval[Z_AXIS]) + 1) as i64
        }
        else {
            -1 * ((self.maxval[X_AXIS] - self.minval[X_AXIS]) + 1) as i64 * ((self.maxval[Y_AXIS] - self.minval[Y_AXIS]) + 1) as i64 * ((self.maxval[Z_AXIS] - self.minval[Z_AXIS]) + 1) as i64
        }
    }

    // Given four points on an axis (e.g. x): x1-1, x1-2 & x2-1, x2-2,
    // where x1-1 and x2-2 repsent the min and max points, respectively,
    // of region 1, and x2-1 and x2-2 likewise do the same for region 2;
    // Find out if the regions overlap, and how they overlap.
    // Returns the tuple (i32::MIN, i32::MIN) if there is no overlap on the
    // given axis.  Otherwise, returns the min and max
    // of the overlapping region 
    // in the order (min_index, max_index)
    fn axis_intersect_check(&self, other_cuboid: &Cuboid, axis: usize) -> (i32, i32) {
        if other_cuboid.maxval[axis] < self.minval[axis] ||
           other_cuboid.minval[axis] > self.maxval[axis] {
            (i32::MIN, i32::MIN)
        }
        else {
            (cmp::max(self.minval[axis], other_cuboid.minval[axis]), cmp::min(self.maxval[axis], other_cuboid.maxval[axis]))
        }
    }

    fn intersect(&self, other_cuboid: &Cuboid) -> Option<Cuboid> {
        let mut intersect: bool = true;
        let mut intersect_idx: [(i32, i32); 3] = [(0i32, 0i32); 3];

        for axis in X_AXIS..=Z_AXIS {
            intersect_idx[axis] = self.axis_intersect_check(other_cuboid, axis);

            if intersect_idx[axis].0 == i32::MIN {
                intersect = false;
                break;
            }
        }

        if !intersect {
            None
        }
        else if intersect_idx[X_AXIS].0 > intersect_idx[X_AXIS].1 ||
                intersect_idx[Y_AXIS].0 > intersect_idx[Y_AXIS].1 ||
                intersect_idx[Z_AXIS].0 > intersect_idx[Z_AXIS].1 {
                    None
                }
        else {
            Some(Cuboid { 
                    is_on: !self.is_on, 
                    minval: [intersect_idx[X_AXIS].0, intersect_idx[Y_AXIS].0, intersect_idx[Z_AXIS].0], 
                    maxval: [intersect_idx[X_AXIS].1, intersect_idx[Y_AXIS].1, intersect_idx[Z_AXIS].1],
                }
            )
        }
    }
}

fn main() {
    let f = File::open("./input.txt").expect("Unable to open input file");
    let f = BufReader::new(f);

    let mut cuboids: Vec<Cuboid> = Vec::with_capacity(512);

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        if line.is_empty() {
            break;
        }

        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() != 2 {
            panic!("Malformed input");
        }

        let mut new_cuboid = Cuboid::new();

        if parts[0].trim() == "on" {
            new_cuboid.is_on = true;
        }

        let axes: Vec<&str> = parts[1].split(",").collect();

        for i in 0..axes.len() {
            let eqn_sides: Vec<&str> = axes[i].split("=").collect();
            let vals: Vec<&str> = eqn_sides[1].split("..").collect();

            new_cuboid.minval[i] = vals[0].parse::<i32>().unwrap();
            new_cuboid.maxval[i] = vals[1].parse::<i32>().unwrap();            
        }

        cuboids.push(new_cuboid);
    }

    let mut lit_cubes: i64 = 0;

    let mut newcubes: Vec<Cuboid> = Vec::with_capacity(1024);

    for i in 0..cuboids.len() {
        let mut mergecubes: Vec<Cuboid> = Vec::with_capacity(128);

        if cuboids[i].is_on {
            mergecubes.push(cuboids[i]);
        }

        for j in 0..newcubes.len() {
            let intersect = newcubes[j].intersect(&cuboids[i]);

            if intersect.is_some() {
                mergecubes.push(intersect.unwrap());
            }
        }

        for j in 0..mergecubes.len() {
            newcubes.push(mergecubes[j]);
        }
    }

    for i in 0..newcubes.len() {
        lit_cubes += newcubes[i].cubes();
    }

    println!("After processing, there are {} lit cubes", lit_cubes);
}
