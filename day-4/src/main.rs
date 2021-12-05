use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::LinkedList;
use std::time::{Duration, Instant};

#[derive(Debug,Clone)]
struct BingoBoard {
    board_num: usize,
    numbers: HashMap<i16, usize>,
    called: u32
}

impl BingoBoard {
    fn from_stdin(stdin: &std::io::Stdin, board_num: usize) -> Option<BingoBoard> {
        let mut new_board = BingoBoard { board_num: board_num, numbers: HashMap::with_capacity(25), called: 0 };
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

fn first_winning_board(numbers: &mut LinkedList<i16>,boards: &mut Vec<BingoBoard>) -> Option<(usize,usize,i32,i16)> {
    while let Some(number) = numbers.front() {
        for i in 0..boards.len() {
            if boards[i].call_number(*number) {
                return Some((i,boards[i].board_num,boards[i].score(),*number));
            }
        }
        // All boards updated
        numbers.pop_front();
    }
    return None;
}

fn main() {
    let start = Instant::now();
    let stdin = io::stdin();
    let mut numbers: LinkedList<i16> = stdin.lock().lines().next().unwrap().unwrap().split(",").flat_map(|s| s.trim().parse::<i16>()).collect();

    let mut boards: Vec<BingoBoard> = vec!();
    while let Some(board) = BingoBoard::from_stdin(&stdin, boards.len() + 1) {
        boards.push(board);
    }

    let mut winners: Vec<(usize,usize,i32,i16)> = vec!();
    let mut first: bool = true;
    while boards.len() > 0 {
        if let Some((winner_idx,board_num,unchecked_sum,last_number)) = first_winning_board(&mut numbers, &mut boards) {
            winners.push((winner_idx,board_num,unchecked_sum,last_number));   // save winners
            boards.remove(winner_idx);  // remove the winner from future numbers, we know it won already
            if first {
                println!("Part 1\r\n{}", "-".repeat(10));
                println!("Winning board #{}, Unchecked Sum: {}, Last number: {}, Score: {}\r\n"
                            ,board_num, unchecked_sum, last_number, unchecked_sum * (last_number as i32));
                first = false;
            }

            if boards.len() == 0 {
                println!("Part 2\r\n{}", "-".repeat(10));
                println!("Last Winning board #{}, Unchecked Sum: {}, Last number: {}, Score: {}\r\n"
                            ,board_num, unchecked_sum, last_number, unchecked_sum * (last_number as i32));
            }
        }
        else
        {
            println!("Sad panda...no winner found");
        }
    }
    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
}
