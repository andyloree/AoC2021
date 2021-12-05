use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug,Clone)]
struct BingoBoard {
    numbers: HashMap<i16, usize>,
    called: u32
}

impl BingoBoard {
    fn from_stdin(stdin: &std::io::Stdin) -> Option<BingoBoard> {
        let mut new_board = BingoBoard { numbers: HashMap::with_capacity(25), called: 0 };
        stdin.lock().lines().skip(1);
        // fill board
        for row in 0..6 {
            let mut line: String = String::new();
            if stdin.read_line(&mut line).expect("stdin error") == 0 {
                return None;
            }
            line.split(" ").flat_map(|val| val.trim().parse::<i16>()).enumerate().for_each(|(i,val)| {
                new_board.numbers.insert(val, (row - 1) * 5 + i);
            });
        }
        return Some(new_board);
    }


    fn call_number(&mut self, number: i16) -> bool {
        if let Some(val) = self.numbers.get(&number) {
            self.called |= 1 << val;
        }
        // win condition
        if self.called & 0x1F == 0x1F            || // row 1
           self.called & 0x3E0 == 0x3E0          || // row 2
           self.called & 0x7C00 == 0x7C00        || // row 3
           self.called & 0xF8000 == 0xF8000      || // row 4
           self.called & 0x1F00000 == 0x1F00000  || // row 5
           self.called & 0x1084210 == 0x1084210  || // col 5
           self.called & 0x842108 == 0x842108    || // col 4
           self.called & 0x421084 == 0x421084    || // col 3
           self.called & 0x210842 == 0x210842    || // col 2
           self.called & 0x108421 == 0x108421    {  // col 1
            return true;
        }
        return false;
    }

    fn score(&self) -> i32 {
        let mut score = 0;
        // map numbers into positional index for scoring
        let mut idx_numbers: [i16; 25] = [0; 25];
        for (number, index) in self.numbers.iter() {
            idx_numbers[(*index as usize)] = *number;
        }
        // iterate bits, score those not set
        for bit_index in 0..25 {
            let mask = 1 << bit_index;
            if self.called & mask == 0 {
                score += idx_numbers[bit_index] as i32;
            }
        }
        return score;
    }
}

fn first_winning_board(numbers: Vec<i16>,mut boards: Vec<BingoBoard>) -> Option<(usize,i32,i16)> {
    for number in numbers {
        for i in 0..boards.len() {
            if boards[i].call_number(number) {
                let score = boards[i].score();
                return Some((i,score,number));
            }
        }
    }
    return None;
}

fn main() {
    let stdin = io::stdin();
    let numbers: Vec<i16> = stdin.lock().lines().next().unwrap().unwrap().split(",").flat_map(|s| s.trim().parse::<i16>()).collect();

    let mut boards: Vec<BingoBoard> = vec!();
    while let Some(board) = BingoBoard::from_stdin(&stdin) {
        boards.push(board);
    }
    println!("Number of boards: {0}", boards.len());

    println!("Part 1\r\n{}", "-".repeat(10));
    if let Some((winner_idx,unchecked_sum,last_number)) = first_winning_board(numbers, boards) {
        println!("Winning board #{}, Unchecked Sum: {}, Last number: {}, Score: {}"
                    ,winner_idx + 1, unchecked_sum, last_number, unchecked_sum * (last_number as i32));
    }
    else
    {
        println!("Sad panda...no winner found");
    }
}
