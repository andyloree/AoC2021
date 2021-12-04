use std::io::{self, BufRead};

#[derive(Debug)]
struct BingoBoard {
    board: [(i16,bool); 25]
}

impl BingoBoard {
    fn from_stdin(stdin: &std::io::Stdin) -> Option<BingoBoard> {
        let mut new_board = BingoBoard { board: [(0,false); 25]};
        stdin.lock().lines().skip(1);
        // fill board
        for row in 0..6 {
            let mut line: String = String::new();
            if stdin.read_line(&mut line).expect("stdin error") == 0 {
                return None;
            }
            line.split(" ").flat_map(|val| val.parse::<i16>()).enumerate().for_each(|(i,val)| {
                new_board.board[(row - 1) * 5 + i] = (val,false);
            });
        }
        return Some(new_board);
    }

    fn call(number: i16) -> bool {
        return true;
    }
}


fn main() {
    let stdin = io::stdin();
    let numbers: Vec<i16> = stdin.lock().lines().next().unwrap().unwrap().split(",").flat_map(|s| s.parse::<i16>()).collect();

    let mut boards: Vec<BingoBoard> = vec!();
    while let Some(board) = BingoBoard::from_stdin(&stdin) {
        boards.push(board);
    }
    println!("Number of boards: {0}", boards.len());

    //boards.into_iter().filter(|&board| board.call(1)).collect();
    //numbers.into_iter().filter(|number| boards. )

}
