#![allow(dead_code)]
#![allow(unused_imports)]
#[macro_use] extern crate primitive_enum;

use std::cmp;

// CONFIG
const ROOM_SIZE: usize = 2;
const AP_COUNT: usize = 8;
const ROOMS: usize = 4;

// ROOM & TYPE
primitive_enum! { APType usize ;
    AMBER,
    BRONZE,
    COPPER,
    DESERT,
}

const ENERGY: [u64; 4] = [1, 10, 100, 1000];

const HALL: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];
const DOORWAY: [usize; ROOMS] = [2, 4, 6, 8];

const NONE: usize = usize::MAX;

// INPUT
const APODS: [APType; AP_COUNT] = [
    APType::BRONZE, 
    APType::BRONZE,
    APType::DESERT,
    APType::DESERT,
    APType::COPPER,
    APType::AMBER,
    APType::AMBER,
    APType::COPPER,
];

enum Dest {
    Hall(usize),
    Room,
}

#[derive(Debug, PartialEq)]
enum MoveInfo {
    BadMove,
    NoMoves,
    Ok,
    DestBlocked,
    DoorBlocked,
    PathBlocked,
    RoomBlocked,
    InRoomDone,
    Win,
    Draw,
}

primitive_enum! { State usize ;
    START,
    HALL,
    MYROOM,
    BLOCKED,
}

#[derive(Clone, Copy)]
struct Board {
    spaces: [usize; 11],
    score: u64,
    state: [State; AP_COUNT],
    ppos: [usize; AP_COUNT],
    turn: [usize; AP_COUNT],
}

impl Board {
    fn new() -> Board {
        let mut b = Board {
            spaces: [NONE; 11],
            score: 0,
            state: [State::START; AP_COUNT],
            ppos: [NONE; AP_COUNT],
            turn: [0; AP_COUNT],
        };

        for i in 0..AP_COUNT {
            if Board::player_to_room(i) == Board::player_to_start(i) &&
               (i / ROOMS) + 1 == ROOM_SIZE {

                b.state[i] = State::MYROOM;
                b.turn[i] = 2;
            }
            b.ppos[i] = Board::player_to_start(i);
        }

        b
    }

    #[inline(always)]
    fn player_to_start(player: usize) -> usize {
        player % ROOMS        
    }

    #[inline(always)]
    fn player_to_room(player: usize) -> usize {
        let aidx = player % ROOMS;
        APODS[aidx] as usize
    }

    /*
    #[inline(always)]
    fn starts_behind(player: usize) -> usize {
        if player < ROOMS {
            NONE
        }
        else {
            player % ROOMS
        }
    }
    */

    #[inline(always)]
    fn steps_to_hall(player: usize) -> usize {
        (player / ROOMS) + 1
    }

    #[inline(always)]
    fn steps_into_room(&self, player: usize) -> usize {
        let room = Board::player_to_room(player);
        let mut occupants: usize = 0;

        for i in 0..AP_COUNT {
            if player == i {
                continue;
            }

            if self.state[i] == State::MYROOM && 
               self.ppos[i] == room {
                   occupants += 1;
            }
        }
        ROOM_SIZE - occupants
    }

    #[inline(always)]
    fn is_my_room_available(&self, player: usize) -> bool {
        for i in 0..AP_COUNT {
            if player == i {
                continue;
            }

            if self.state[i] == State::START &&
               Board::player_to_start(i) == Board::player_to_room(player) &&
               APODS[i] != APODS[player] {
                   return false;
            }
        }
        true
    }

    #[inline(always)]
    fn is_player_done(&self, player: usize) -> bool {
        if self.state[player] == State::MYROOM {
            true
        }
        else {
            false
        }
    }

    fn is_door_blocked(&self, player: usize) -> bool {
        if player < ROOMS || self.state[player] != State::START {
            false
        }
        else {
            let mut in_front = player - ROOMS;
            while in_front >= 0 {
                if self.state[in_front] == State::START {
                    return true
                }
            }
            true
        }
    }

    fn first_to_room(&self, player: usize) -> bool {
        let dest_room = Board::player_to_room(player);

        for i in 0..AP_COUNT {
            if i == player {
                continue;
            }
            
            if self.state[i] == State::MYROOM &&
               APODS[i] as usize == dest_room {
                    return false;
            } 
        }
        true
    }

    fn is_space_open(&self, player: usize, space: &Dest) -> MoveInfo {
        match space {
            Dest::Hall(dest_space) => {
                if self.spaces[*dest_space] == NONE {
                    MoveInfo::Ok
                }
                else {
                    MoveInfo::DestBlocked
                }
            },
            Dest::Room => {
                for i in 0..AP_COUNT {
                    if player == i {
                        continue;
                    }

                    if self.state[i] == State::START &&
                       Board::player_to_start(i) == Board::player_to_room(player) &&
                       APODS[i] != APODS[player] {
                           return MoveInfo::RoomBlocked;
                       }
                }
                MoveInfo::Ok
            }
        }        
    }

    fn is_path_open(&self, player: usize, space: &Dest) -> (MoveInfo, usize) {
        let si = self.is_space_open(player, &space);
        if si != MoveInfo::Ok {
            return (si, 0);
        }

        let start: usize;
        let end: usize;

        let mut dist: usize = 0;

        match space {
            Dest::Hall(dest_space) => {
                if self.state[player] != State::START {
                    return (MoveInfo::BadMove, 0);
                }
                
                if self.is_door_blocked(player) {
                    return (MoveInfo::DoorBlocked, 0);
                }
                else {
                    start = DOORWAY[Board::player_to_start(player)];
                    end = *dest_space;
                    dist += Board::steps_to_hall(player);
                }
            },
            Dest::Room => {
                if !self.is_my_room_available(player) {
                    return (MoveInfo::RoomBlocked, 0);
                }

                if self.state[player] == State::START {
                    if self.is_door_blocked(player) {
                        return (MoveInfo::DoorBlocked, 0);
                    }
                    else {
                        start = DOORWAY[Board::player_to_start(player)];
                        end = DOORWAY[Board::player_to_room(player)];
                        dist += Board::steps_to_hall(player);
                        dist += self.steps_into_room(player);
                    }
                }
                else if self.state[player] == State::HALL {
                    start = self.ppos[player];
                    end = DOORWAY[Board::player_to_room(player)];
                    dist += self.steps_into_room(player);
                }
                else {
                    return (MoveInfo::BadMove, 0);
                } 
            }
        }

        for i in cmp::min(start+1,end+1)..cmp::max(start,end) {
            if self.spaces[i] != NONE {
                return (MoveInfo::PathBlocked, 0);
            }
        }
        dist += cmp::max(start,end) - cmp::min(start,end);
        (MoveInfo::Ok, dist)
    }

    fn moves(&self, player: usize) -> usize {
        let mut move_count: usize = 0;

        if self.is_path_open(player, &Dest::Room).0 != MoveInfo::Ok {
            move_count += 1;
        }

        if self.state[player] == State::START {
            for i in HALL {
                let pr = self.is_path_open(player, &Dest::Hall(i));
                if pr.0 == MoveInfo::Ok {
                    move_count += 1;
                }
                else if pr.0 == MoveInfo::DoorBlocked {
                    break;
                }
            }
        }

        move_count
    }

    fn move_hall_players_to_room(&mut self) -> u64 {
        let mut moved = true;
        let mut energy: u64 = 0;

        while moved {
            moved = false;
            for i in 0..AP_COUNT {
                if self.state[i] == State::HALL {
                    let mr = self.mov(i, &Dest::Room);
                    if mr.0 == MoveInfo::Ok {
                        moved = true;
                        energy += mr.1 as u64 * ENERGY[Board::player_to_room(i)];
                    }
                }
            }
        }

        energy
    }

    fn move_room_players_to_room(&mut self) -> u64 {
        let mut moved = true;
        let mut energy: u64 = 0;

        while moved {
            moved = false;
            for i in 0..AP_COUNT {
                if self.state[i] == State::START {
                    let mr = self.mov(i, &Dest::Room);
                    if mr.0 == MoveInfo::Ok {
                        moved = true;
                        energy += mr.1 as u64 * ENERGY[Board::player_to_room(i)];
                    }
                }
            }
        }

        energy
    }

    fn mov(&mut self, player: usize, dest: &Dest) -> (MoveInfo, u64) {
        println!("Moving player {}", player);
        let pi = self.is_path_open(player, dest);
        let mi: MoveInfo;

        if pi.0 != MoveInfo::Ok {
            return (pi.0, 0);
        }

        let dist = pi.1;

        match dest {
            Dest::Hall(dest_space) => {
                if self.state[player] != State::START {
                       return (MoveInfo::BadMove, 0);
                }

                self.spaces[*dest_space] = player;
                self.ppos[player] = *dest_space;
                self.state[player] = State::HALL;
                mi = MoveInfo::Ok;
            },
            Dest::Room => {
                if self.state[player] == State::HALL {
                    self.spaces[self.ppos[player]] = NONE;
                }

                self.ppos[player] = APODS[player] as usize;
                self.state[player] = State::MYROOM;
                mi = MoveInfo::Ok;
            }
        }

        (mi, dist as u64 * ENERGY[APODS[player] as usize])
    }

    fn is_win(&self) -> bool {
        for i in 0..AP_COUNT {
            if self.state[i] != State::MYROOM {
                return false
            }
        }
        true
    }

    fn is_cat_game(&self) -> bool {
        let mut done: usize = 0;

        for i in 0..AP_COUNT {
            if self.state[i] != State::MYROOM && self.moves(i) > 0 {
                return false;
            }
            else if self.state[i] == State::MYROOM {
                done += 1;
            }
        }

        if done == AP_COUNT {
            false
        }
        else {
            true
        }
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

    }
}