const AMBER: usize = 0;
const BRONZE: usize = 1;
const COPPER: usize = 2;
const DESERT: usize = 3;

const ENERGY: [u64; 4] = [1, 10, 100, 1000];

const SAFE_SPOTS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];
const ROOM_SPOTS: [usize; 4] = [2, 4, 6, 8];

const SPOTS: usize = 11;

/*
#############
#...........#
###B#B#D#D###
  #C#A#A#C#
  #########
*/

#[derive(Clone)]
struct Amphipod {
    typ: usize,
    pos: usize,
    locked: bool,
    wait: usize,    
    in_room: bool,
    start_pos: usize,
    energy_used: u64,
}

#[inline(always)]
fn spot_to_room(spot: usize) -> usize {
    (spot - 2) / 2
}

fn main() {
    let mut min_energy: u64 = u64::MAX;

    
    for i in 0..4 {
        let mut amphipods: Vec<Amphipod> = Vec::with_capacity(8);
        let mut occupied: [usize; 11] = [usize::MAX; 11];
        let mut room_occupants: [usize; 4] = [2; 4];
        
        reset_amphipods(&mut amphipods);
    }
    
    fn move_out(amphipods: &mut Vec<Amphipod>, amphipod_idx: usize, dest_space: usize, room_occupants: &mut [usize; 4], occupied: &mut [usize; 11]) -> bool {
        if !amphipods[amphipod_idx].in_room {
            return false
        }

        let path = path_out(amphipods, amphipod_idx, dest_space, occupied);
        if !path.0 {
            false
        }
        else {
            amphipods[amphipod_idx].in_room = false;
            room_occupants[amphipods[amphipod_idx].pos] -= 1;               
            amphipods[amphipod_idx].pos = dest_space;
            occupied[dest_space] = amphipod_idx;
            amphipods[amphipod_idx].locked = true;
            amphipods[amphipod_idx].energy_used += ENERGY[amphipods[amphipod_idx].typ] * path.1 as u64;

            true
        }
    }

    fn path_out(amphipods: &Vec<Amphipod>, amphipod_idx: usize, dest_space: usize, occupied: &mut [usize; 11]) -> (bool, usize) {
        let mut start_pos: usize = 0;
        let mut extra_moves: usize = 0;

        if amphipods[amphipod_idx].in_room && amphipods[amphipod_idx].start_pos == 2 &&
           amphipods[amphipods[amphipod_idx].wait].in_room &&
           !amphipods[amphipods[amphipod_idx].wait].locked {
                    return (false, 0)
        }
        else if (amphipods[amphipod_idx].in_room && amphipods[amphipod_idx].start_pos == 2 &&
               !amphipods[amphipods[amphipod_idx].wait].in_room) || 
               (amphipods[amphipod_idx].in_room && amphipods[amphipod_idx].start_pos == 2 &&
                amphipods[amphipods[amphipod_idx].wait].in_room &&
                amphipods[amphipods[amphipod_idx].wait].locked) {
                    start_pos = (amphipods[amphipod_idx].pos * 2) + 2;
                    extra_moves = 2;
        }
        else if amphipods[amphipod_idx].in_room && amphipods[amphipod_idx].start_pos == 1 {
            start_pos = (amphipods[amphipod_idx].pos * 2) + 2;
            extra_moves = 1
        }
        else {
            start_pos = amphipods[amphipod_idx].pos;
        }

        let mut moves: usize = 0;

        if dest_space <= start_pos {
            moves = start_pos - dest_space;
            for i in dest_space..start_pos {
                if occupied[i] != usize::MAX {
                    return (false, 0)
                }
            }
            (true, moves + extra_moves)
        }
        else {
            moves = dest_space - start_pos;
            for i in start_pos+1..=dest_space {
                if occupied[i] != usize::MAX {
                    return (false, 0)
                }
            }
            (true, moves + extra_moves)
        }
    }

    fn move_in(amphipods: &mut Vec<Amphipod>, amphipod_idx: usize, room_occupants: &mut [usize; 4], occupied: &mut [usize; 11]) -> bool {
        if amphipods[amphipod_idx].in_room {
            return false
        }

        let path = path_in(amphipods, amphipod_idx, room_occupants, occupied);
        if !path.0 {
            false
        }
        else {
            amphipods[amphipod_idx].in_room = true;
            room_occupants[amphipods[amphipod_idx].typ] += 1;               
            occupied[amphipods[amphipod_idx].pos] = usize::MAX;
            amphipods[amphipod_idx].pos = amphipods[amphipod_idx].typ;
            amphipods[amphipod_idx].locked = true;
            amphipods[amphipod_idx].energy_used += ENERGY[amphipods[amphipod_idx].typ] * path.1 as u64;

            true
        }
    }

    fn path_in(amphipods: &Vec<Amphipod>, amphipod_idx: usize, room_occupants: &mut [usize; 4], occupied: &mut [usize; 11]) -> (bool, usize) {
        if amphipods[amphipod_idx].in_room || !amphipods[amphipod_idx].locked {
            return (false, 0)
        }

        let extra_moves = 2 - room_occupants[amphipods[amphipod_idx].typ];
        let dest_space = ROOM_SPOTS[amphipods[amphipod_idx].typ];

        let mut moves: usize = 0;

        if dest_space <= amphipods[amphipod_idx].pos {
            moves = amphipods[amphipod_idx].pos - dest_space;
            for i in dest_space..amphipods[amphipod_idx].pos {
                if occupied[i] != usize::MAX {
                    return (false, 0)
                }
            }
            (true, moves + extra_moves)
        }
        else {
            moves = dest_space - amphipods[amphipod_idx].pos;
            for i in amphipods[amphipod_idx].pos+1..=dest_space {
                if occupied[i] != usize::MAX {
                    return (false, 0)
                }
            }
            (true, moves + extra_moves)
        }
    }

    fn reset_amphipods(amphipods: &mut Vec<Amphipod>) {
        amphipods.clear();
        amphipods.push(Amphipod { typ: BRONZE, pos: 0, locked: false, wait: 0, in_room: true, start_pos: 1, energy_used: 0 });
        amphipods.push(Amphipod { typ: BRONZE, pos: 1, locked: false, wait: 0, in_room: true, start_pos: 1, energy_used: 0 });
        amphipods.push(Amphipod { typ: DESERT, pos: 2, locked: false, wait: 0, in_room: true, start_pos: 1, energy_used: 0 });
        amphipods.push(Amphipod { typ: DESERT, pos: 3, locked: false, wait: 0, in_room: true, start_pos: 1, energy_used: 0 });
        amphipods.push(Amphipod { typ: COPPER, pos: 0, locked: false, wait: 0, in_room: true, start_pos: 2, energy_used: 0 });
        amphipods.push(Amphipod { typ: AMBER, pos: 1, locked: false, wait: 1, in_room: true, start_pos: 2, energy_used: 0 });
        amphipods.push(Amphipod { typ: AMBER, pos: 2, locked: false, wait: 2, in_room: true, start_pos: 2, energy_used: 0 });
        amphipods.push(Amphipod { typ: COPPER, pos: 3, locked: false, wait: 3, in_room: true, start_pos: 2, energy_used: 0 });
    }

    println!("Hello, world!");
}
