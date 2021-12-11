use std::io::{self, BufRead};
use std::time::{Instant};
use std::collections::VecDeque;

fn parse_line(line: &String) -> (bool,i32,i64) {
    let mut depth: VecDeque<char> = VecDeque::new();
    let illegal_to_points = |close: char| -> i32 { 
        match close {
            ')' => return 3,
            ']' => return 57,
            '}' => return 1197,
            '>' => return 25137,
             _  => unreachable!()
        }
    };
    let is_correct_close = |open: char, close: char| -> bool { 
        if open == '(' && close == ')' { return true; }
        if open == '<' && close == '>' { return true; }
        if open == '[' && close == ']' { return true; }
        if open == '{' && close == '}' { return true; }
        return false;
    };

    for c in line.chars() {
        match c {
            '('|'['|'{'|'<' => depth.push_front(c),
            ')'|']'|'}'|'>' => {
                let open = depth.pop_front();
                if open.is_none() || !is_correct_close(open.unwrap(),c) {
                    return (false, illegal_to_points(c),0);
                }
            },
            _ => unreachable!()
        }
    }

    let incomplete_to_points = |open: char| -> i64 {
        match open {
            '(' => return 1,
            '[' => return 2,
            '{' => return 3,
            '<' => return 4,
             _  => unreachable!()
        }
    };

    return (true, 0, depth.into_iter().fold(0,|acc: i64, open: char| acc * 5 + incomplete_to_points(open)));
}

fn main() {
    let start = Instant::now();
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().flatten().collect();

    let total_points = lines.iter().map(|line| {
        let (_complete, points, _incomplete_points) =  parse_line(line);
        return points;
    }).sum::<i32>();
    
    println!("Part 1\r\n{}", "-".repeat(10));
    println!("Points: {}\r\n", total_points);

    let mut incomplete_scores: Vec<i64> = lines.iter().map(|line| {
        let (_complete, _points, incomplete_points) =  parse_line(line);
        return incomplete_points;
    }).filter(|points| *points != 0).collect();
    incomplete_scores.sort();

    println!("Part 2\r\n{}", "-".repeat(10));
    println!("Middle incomplete score: {:?}\r\n", incomplete_scores[incomplete_scores.len() / 2]);

    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
}