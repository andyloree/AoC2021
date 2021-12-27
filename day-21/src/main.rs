use std::io::{self, BufRead};
use std::time::{Instant};

struct Player {
    name: String,
    start_at: u8,
    current_position: u8,
    score: u16
}

impl Player {
    fn move_spaces(&mut self, num_spaces: u16) {
        self.current_position = ((self.current_position as u16 + num_spaces) % 10) as u8;
        if self.current_position == 0 {
            self.current_position = 10;
        }
        self.score += self.current_position as u16;
    }
}

fn play_deterministic(player1: &mut Player, player2: &mut Player, winning_score: u16) -> u16 {
    let mut dice: u16 = 1;
    let mut turn: u16 = 1;

    let mut roll_dice = || -> u16 {
        let mut sum = 0;
        for _ in 0..3 {
            sum += dice;
            if dice == 100 {
                dice = 1;
            }
            else {
                dice += 1;
            }
        }
        return sum;
    };

    loop
    {
        let num_spaces = roll_dice();
        if turn % 2 == 1 {
            player1.move_spaces(num_spaces);
            println!("Player 1, rolled {}, on space: {}, score: {}, ", num_spaces, player1.current_position, player1.score);
        }
        else {
            player2.move_spaces(num_spaces);
            println!("Player 2, rolled {}, on space {}, score: {}, ", num_spaces, player2.current_position, player2.score)
        }

        if player1.score >= winning_score || player2.score >= winning_score {
            break;
        }
        turn += 1;
    }

    return turn * 3; // num dice roles
}

fn main() {
    let start = Instant::now();
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().flatten().collect();
    let player1_start = lines[0].replacen("Player 1 starting position: ", "", 1).parse::<u8>().unwrap();
    let player2_start = lines[1].replacen("Player 2 starting position: ", "", 1).parse::<u8>().unwrap();

    let mut player1 = Player{name: "Player 1".to_string(), start_at: player1_start, current_position: player1_start, score: 0};
    let mut player2 = Player{name: "Player 2".to_string(), start_at: player2_start, current_position: player2_start, score: 0};

    let num_dice_rolls = play_deterministic(&mut player1, &mut player2, 1000);
    println!("Part 1\r\n{}", "-".repeat(10));
    println!("Num dice rolls: {}", num_dice_rolls);
    println!("Min score: {}", player1.score.min(player2.score));
    println!("Combined: {}", player1.score.min(player2.score) as u32 * num_dice_rolls as u32);

    println!("Part 2\r\n{}", "-".repeat(10));
    // todo

    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
}