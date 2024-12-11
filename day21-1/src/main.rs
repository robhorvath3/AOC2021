const P1_START: u32 = 7;
const P2_START: u32 = 4;

const SPACES: u32 = 10;
const WIN_AMT: u32 = 1000;

struct Player {
    score: u32,
    pos: u32,
}

impl Player {
    fn new(pos: u32) -> Player {
        Player {
            score: 0,
            pos: pos,
        }
    }
}

struct Die100 {
    val: u32,
    roll_count: u32,
}

impl Die100 {
    fn new() -> Die100 {
        Die100 {
            val: 0,
            roll_count: 0,
        }
    }

    fn roll(&mut self) -> u32 {
        if self.val == 100 {
            self.val = 1;
        }
        else {
            self.val += 1;
        }
        self.roll_count += 1;
        self.val
    }
}

fn main() {
    let mut p1 = Player::new(P1_START);
    let mut p2 = Player::new(P2_START);
    let mut die100 = Die100::new();

    loop {
        let mut p1_rolls = die100.roll();
        p1_rolls += die100.roll();
        p1_rolls += die100.roll();

        p1.pos = (p1.pos + p1_rolls) % SPACES;
        if p1.pos == 0 {
            p1.pos = SPACES;
        }

        p1.score += p1.pos;

        if p1.score > WIN_AMT {
            break;
        }

        let mut p2_rolls = die100.roll();
        p2_rolls += die100.roll();
        p2_rolls += die100.roll();

        p2.pos = (p2.pos + p2_rolls) % SPACES;
        if p2.pos == 0 {
            p2.pos = SPACES;
        }

        p2.score += p2.pos;

        if p2.score > WIN_AMT {
            break;
        }
    }
    
    let mut lowscore: u32 = 0;

    if p1.score < p2.score {
        println!("Player 2 won with a score of {}", p2.score);
        lowscore = p1.score;
        println!("Player 1 lost with a score of {}", p1.score);
    }
    else {
        println!("Player 1 won with a score of {}", p1.score);
        lowscore = p2.score;
        println!("Player 2 lost with a score of {}", p2.score);
    }

    println!("The 100 sided deterministic die was rolled {} times", die100.roll_count);

    println!("The product obtained from the losing score and the number of times the die was rolled is {}", die100.roll_count * lowscore);
}
