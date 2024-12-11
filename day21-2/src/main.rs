const P1_START: u64 = 7;
const P2_START: u64 = 2;

const SPACES: u64 = 10;
const WIN_AMT: u64 = 21;

const ROLL_FREQ: [u64; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

fn main() {
    let mut p1_wins: u64 = 0;
    let mut p2_wins: u64 = 0;

    do_rounds(&mut p1_wins, &mut p2_wins, P1_START, P2_START, 0, 0, 1);

    if p1_wins > p2_wins {
        println!("Player 1 wins.");
    }
    else {
        println!("Player 2 wins.");
    }

    println!("After executing, player 1 has {} wins and player 2 has {} wins", p1_wins, p2_wins);    
}

fn do_rounds(p1_wins: &mut u64, p2_wins: &mut u64, p1_pos: u64, p2_pos: u64, p1_score: u64, p2_score: u64, factor: u64) {

    for p1_roll in 3..=9 {
        
        let new_pos1 = {
            if (p1_pos + p1_roll) % SPACES == 0 {
                SPACES
            }
            else {
                (p1_pos + p1_roll) % SPACES
            }
        };

        if p1_score + new_pos1 >= WIN_AMT {
            *p1_wins += factor * ROLL_FREQ[p1_roll as usize];
            continue;
        }

        for p2_roll in 3..=9 {

            let new_pos2 = {
                if (p2_pos + p2_roll) % SPACES == 0 {
                    SPACES
                }
                else {
                    (p2_pos + p2_roll) % SPACES
                }
            };

            if p2_score + new_pos2 >= WIN_AMT {
                *p2_wins += factor * ROLL_FREQ[p1_roll as usize] * ROLL_FREQ[p2_roll as usize];
            }
            else {
                do_rounds(p1_wins, p2_wins, new_pos1, new_pos2, p1_score + new_pos1, p2_score + new_pos2, factor * ROLL_FREQ[p1_roll as usize] * ROLL_FREQ[p2_roll as usize]);
            }
        }
    }
}