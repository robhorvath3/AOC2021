#![allow(dead_code)]
#![allow(unused_imports)]

use std::cmp;

const AMBER: usize = 0;
const BRONZE: usize = 1;
const COPPER: usize = 2;
const DESERT: usize = 3;

const ENERGY: [u64; 4] = [1, 10, 100, 1000];

const HALL_SPACE: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];
const DOORWAY_SPACE: [usize; 4] = [2, 4, 6, 8];

const EXIT_AFTER_WHO: [Option<usize>; 8] = [None, None, None, None, Some(0), Some(1), Some(2), Some(3)];

const NONE: usize = usize::MAX;
const STATE_START_FIRST: usize = 0;
const STATE_START_LAST: usize = 1;
const STATE_LOCKED: usize = 2;
const STATE_DONE: usize = 3;

struct APod {
    idx: usize,
    typ: usize,
    start: usize,
}

const APODS: [APod; 8] = [
    APod { idx: 0, typ: BRONZE, start: 0 },
    APod { idx: 1, typ: BRONZE, start: 1 },
    APod { idx: 2, typ: DESERT, start: 2 },
    APod { idx: 3, typ: DESERT, start: 3 },
    APod { idx: 4, typ: COPPER, start: 0 },
    APod { idx: 5, typ: AMBER, start: 1 },
    APod { idx: 6, typ: AMBER, start: 2 },
    APod { idx: 7, typ: COPPER, start: 3 },
];

enum Space {
    Hall(usize),
    Room,
}

#[derive(Debug, PartialEq)]
enum MoveInfo {
    BadMove,
    NoMoves,
    NoSubMoves,
    NoTurns,
    Ok,
    SpaceOccupied,
    DoorBlocked,
    PathBlocked,
    RoomBlocked,
    InRoomDone,
    Win,
    Draw,
}

#[derive(Clone, Copy)]
struct Board {
    spaces: [usize; 11],
    score: u64,
    state: [usize; 8],
    ppos: [usize; 8],
    turns: [usize; 8],
}

impl Board {
    fn new() -> Board {
        let mut b = Board {
            spaces: [NONE; 11],
            score: 0,
            state: [NONE; 8],
            ppos: [NONE; 8],
            turns: [0; 8],
        };

        for i in 0..8 {
            b.ppos[i] = APODS[i].start;
            if i < 4 {
                b.state[i] = STATE_START_FIRST;                
            }
            else {
                b.state[i] = STATE_START_LAST;
            }
        }

        b
    }

    fn is_win(&self) -> bool {
        for i in 0..8 {
            if self.state[i] != STATE_DONE {
                return false
            }
        }
        true
    }

    fn is_cat_game(&self) -> bool {
        let mut done: usize = 0;

        for i in 0..8 {
            if self.state[i] != STATE_DONE && self.moves(i) > 0 {
                return false;
            }
            else if self.state[i] == STATE_DONE {
                done += 1;
            }
        }

        if done == 8 {
            false
        }
        else {
            true
        }
    }

    fn is_player_done(&self, player: usize) -> bool {
        if self.state[player] == STATE_DONE {
            true
        }
        else {
            false
        }
    }

    fn is_path_open(&self, player: usize, space: &Space) -> MoveInfo {
        let si = self.is_space_open(player, &space);
        if si != MoveInfo::Ok {
            return si;
        }

        let start: usize;
        let end: usize;

        match space {
            Space::Hall(dest_space) => {
                if self.is_door_blocked(player) {
                    return MoveInfo::DoorBlocked;
                }
                else {
                    start = DOORWAY_SPACE[APODS[player].start];
                    end = *dest_space;
                }
            },
            Space::Room => {
                start = {
                    if player < 4 {
                        player
                    }
                    else {
                        player - 4
                    }
                };
                end = DOORWAY_SPACE[APODS[player].typ];
            }
        }

        for i in cmp::min(start+1,end+1)..cmp::max(start,end) {
            if self.spaces[i] != NONE {
                return MoveInfo::PathBlocked;
            }
        }

        MoveInfo::Ok
    }

    fn dist(&self, player: usize, space: &Space) -> u64 {
        let start: usize;
        let end: usize;
        let mut dist: u64 = 0;

        if self.state[player] == STATE_DONE {
            return 0;
        }

        match space {
            Space::Hall(dest_space) => {
                start = DOORWAY_SPACE[APODS[player].start];
                end = *dest_space;

                match self.state[player] {
                    STATE_START_FIRST => {
                        dist += 1;
                    },
                    STATE_START_LAST => {
                        dist += 2;
                    },
                    _ => {},
                }
            },
            Space::Room => {
                if ((self.state[player] == STATE_START_FIRST &&
                    APODS[player+4].typ == APODS[player].typ) ||
                    self.state[player] == STATE_START_LAST) && 
                    self.ppos[player] == APODS[player].typ {
                        start = 0;
                        end = 0;
                }
                else if self.state[player] == STATE_START_FIRST {
                    dist += 1;
                    start = DOORWAY_SPACE[self.ppos[player]];
                    end = DOORWAY_SPACE[APODS[player].typ];
                }
                else if self.state[player] == STATE_START_LAST {
                    dist += 2;
                    start = DOORWAY_SPACE[self.ppos[player]];
                    end = DOORWAY_SPACE[APODS[player].typ];
                }
                else {
                    start = self.ppos[player];
                    end = DOORWAY_SPACE[APODS[player].typ];
                }
                    
                if self.first_to_room(player) {
                    dist += 2;
                }
                else {
                    dist += 1;
                }
            }
        }

        dist += cmp::max(start,end) as u64 - cmp::min(start,end) as u64;
        dist
    }

    fn is_door_blocked(&self, player: usize) -> bool {
        if EXIT_AFTER_WHO[player].is_some() {
            let blocker = EXIT_AFTER_WHO[player].unwrap();
            if self.state[blocker] == STATE_START_FIRST {
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    fn first_to_room(&self, player: usize) -> bool {
        let dest_room = {
            if player < 4 {
                player
            }
            else {
                player - 4
            }
        };

        for i in 0..8 {
            if i == player {
                continue;
            }
            
            if self.state[i] == STATE_DONE &&
               APODS[i].typ == dest_room {
                    return false;
            }        
        }
        true
    }

    fn is_space_open(&self, player: usize, space: &Space) -> MoveInfo {
        match space {
            Space::Hall(dest_space) => {
                if self.spaces[*dest_space] == NONE {
                    MoveInfo::Ok
                }
                else {
                    MoveInfo::SpaceOccupied
                }
            },
            Space::Room => {
                for i in 0..8 {
                    if (self.state[i] == STATE_START_FIRST || 
                       self.state[i] == STATE_START_LAST) &&
                       APODS[i].start == APODS[player].typ &&
                       APODS[i].typ != APODS[player].typ {
                           return MoveInfo::RoomBlocked;
                       }
                }
                MoveInfo::Ok
            }
        }        
    }

    fn moves(&self, player: usize) -> usize {
        let mut move_count: usize = 0;

        if self.is_path_open(player, &Space::Room) != MoveInfo::Ok {
            move_count += 1;
        }

        if self.state[player] == STATE_START_FIRST || 
           self.state[player] == STATE_START_LAST {
            for i in HALL_SPACE {
                let pr = self.is_path_open(player, &Space::Hall(i));
                if pr == MoveInfo::Ok {
                    move_count += 1;
                }
                else if pr == MoveInfo::DoorBlocked {
                    break;
                }
            }
        }

        move_count
    }

    fn mov(&mut self, player: usize, dest: &Space) -> (MoveInfo, u64) {
        println!("Moving player {}", player);
        let pi = self.is_path_open(player, dest);
        let mi: MoveInfo;

        if pi != MoveInfo::Ok {
            return (pi, 0);
        }

        let dist = self.dist(player, dest);

        match dest {
            Space::Hall(dest_space) => {
                if self.state[player] != STATE_START_FIRST ||
                   self.state[player] != STATE_START_LAST {
                       return (MoveInfo::BadMove, 0);
                }

                self.spaces[*dest_space] = player;
                self.ppos[player] = *dest_space;
                self.state[player] = STATE_LOCKED;
                mi = MoveInfo::Ok;
            },
            Space::Room => {
                if self.state[player] != STATE_LOCKED &&
                !(((self.state[player] == STATE_START_FIRST && self.is_space_open(player, &Space::Room) == MoveInfo::Ok) ||
                   self.state[player] == STATE_START_LAST) &&
                   APODS[player].typ == APODS[player].start) {
                       return (MoveInfo::BadMove, 0);
                }

                if self.state[player] == STATE_LOCKED {
                    self.spaces[self.ppos[player]] = NONE;
                }

                self.ppos[player] = APODS[player].typ;
                self.state[player] = STATE_DONE;
                mi = MoveInfo::Ok;
            }
        }

        (mi, dist * ENERGY[APODS[player].typ])
    }
}

/*
#############
#...........#
###B#B#D#D###
  #C#A#A#C#
  #########
*/

fn main() {
    let mut low_score: u64 = u64::MAX;

    play(&mut low_score);
    println!("The lowest score was {}", low_score);

    fn play(low_score: &mut u64) {
        let board_orig: Board = Board::new();

        for i in 0..4 {
            let board = board_orig.clone();            
            next_player(board, i, 0, low_score);            
        }
    }

    fn next_player(board: Board, player: usize, score: u64, low_score: &mut u64) -> MoveInfo {
        let mut player = player;
        let mut score = score;

        if board.is_player_done(player) {
            return MoveInfo::InRoomDone;
        }

        if board.is_cat_game() {
            return MoveInfo::Draw;
        }

        if board.moves(player) == 0 {
            return MoveInfo::NoMoves;
        }

        // always try to move home first
        let mut lboard: Board = board.clone();

        let ri = lboard.mov(player, &Space::Room);
        if ri.0 == MoveInfo::Ok {
            return MoveInfo::Ok;
        }
        
        // now try to move any hallway stragglers home
        let mut moved_home: bool = true;

        while moved_home {
            moved_home = false;

            for i in 0..8 {
                if lboard.state[i] != STATE_DONE {
                    let mr = lboard.mov(i, &Space::Room);
                    if mr.0 == MoveInfo::Ok {
                        moved_home = true;
                        lboard.turns[i] = 2;
                        score += mr.1;
                    }
                }
            }
        }

        if lboard.is_win() {
            if score < *low_score {
                *low_score = score;
            }
            return MoveInfo::Win;
        }

        if lboard.turns[player] == 1 {
            for i in 0..8 {
                if player == i {
                    continue;
                }
                if lboard.turns[i] == 0 {
                    player = i;
                    break;
                }
            }
        }

        match lboard.state[player] {
            STATE_START_FIRST | STATE_START_LAST => {
                for i in HALL_SPACE {
                    if lboard.is_space_open(player, &Space::Hall(i)) == MoveInfo::Ok {
                        let mut llboard = lboard.clone();
                        let pi = llboard.mov(player, &Space::Hall(i));
                        if pi.0 == MoveInfo::Ok {
                            lboard.turns[player] = 1;                            
                            let np = next_player(llboard, player, score + pi.1, low_score);
                            match np {
                                MoveInfo::Draw
                            }
                        }
                    }
                }
            },
            STATE_LOCKED => {

            },
            _ => {},
        }        
        MoveInfo::Ok
    }
}