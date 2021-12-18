use std::io::{self, BufRead};
use std::time::{Instant};
use std::ops;
use std::collections::BinaryHeap;

fn peek_char(s: &mut String) -> char {
    return s.chars().nth(0).unwrap();
}

fn pop_char(s: &mut String) -> Option<char> {
    if s.len() == 0 {
        return None;
    }
    let c = peek_char(s);
    s.drain(..=0);
    return Some(c);
}

#[derive(Debug,Copy,Clone)]
struct Number {
    value: u32,
    depth: usize
}

impl Number {
    fn add(&self,increase_depth: bool) -> Number {
        Number {value: self.value, depth: self.depth + if increase_depth {1} else {0}}
    }
}

#[derive(Debug,Clone)]
struct SnailNumber {
    numbers: Vec<Number>
}

impl SnailNumber {
    fn new() -> Self {
        SnailNumber { numbers: vec!()}
    }

    fn parse(&mut self, line: &mut String) {
        self.numbers.clear();
        self.parse_recur(line, 0);
    }

    fn parse_recur(&mut self, line: &mut String, depth: usize) {
        if peek_char(line) == '[' {
            pop_char(line); // [
            self.parse_recur(line, depth + 1);
            let comma = pop_char(line); // ,
            if comma.is_none() {return};
            assert_eq!(comma,Some(','));
            self.parse_recur(line, depth + 1);
            let close_bracket = pop_char(line); // ]
            assert_eq!(close_bracket,Some(']'));
        }
        else {
            // Must be number
            let mut num_str: String = "".to_string();
            loop {
                match peek_char(line) {
                    ']' | ',' => break,
                    c => {
                        pop_char(line);
                        num_str.push_str(&c.to_string());
                    }
                };
            }
            let number = match num_str.parse::<u32>() {
                Ok(number) => number,
                Err(_e) => unreachable!(),
            };
            self.numbers.push(Number {value: number, depth: depth});
        }
    }

    fn reduce(&mut self) {
        loop {
            // explode
            if let Some(explode_idx) = self.numbers.iter().position(|n| n.depth > 4) {
                let left_idx = explode_idx;
                let right_idx = explode_idx + 1;
                if left_idx > 0 {    // we have a regular number to our left
                    self.numbers[left_idx - 1].value += self.numbers[left_idx].value;
                    self.numbers[left_idx].value = 0;
                    self.numbers[left_idx].depth -= 1;
                }
                else {
                    self.numbers[left_idx].value = 0;
                    self.numbers[left_idx].depth -= 1;
                }

                if right_idx + 1 < self.numbers.len() {    // we have a regular number to our right
                    self.numbers[right_idx + 1].value += self.numbers[right_idx].value;
                    self.numbers.remove(right_idx);
                }
                else {
                    self.numbers.remove(right_idx);
                }
            }
            // split
            else if let Some(split_idx) = self.numbers.iter().position(|n| n.value > 9) {
                let orig_value = self.numbers[split_idx].value;
                self.numbers[split_idx].value = orig_value / 2;
                self.numbers[split_idx].depth += 1;
                self.numbers.insert(split_idx + 1, Number{ value: (orig_value + 1) / 2, depth: self.numbers[split_idx].depth});
            }
            else {
                break;
            }
        }
    }

    fn magnitude(&self) -> u32 {
        let mut numbers: Vec<Number> = self.numbers.clone();
        loop {
            match numbers.iter().map(|n| n.depth).max() {
                None => break,
                Some(max_depth) => {
                    if max_depth == 0 {
                        return numbers[0].value;
                    }
                    match numbers.iter().position(|n| n.depth == max_depth) {
                        None => unreachable!(),
                        Some(left_idx) => {
                            numbers[left_idx].value = numbers[left_idx].value * 3 + numbers[left_idx + 1].value * 2;
                            numbers[left_idx].depth -= 1;
                            numbers.remove(left_idx + 1);
                        }
                    }
                }
            }
        }
        unreachable!();
    }
}

impl ops::Add<SnailNumber> for SnailNumber {
    type Output = SnailNumber;

    fn add(self, rhs: SnailNumber) -> SnailNumber {
        let mut new_snail = SnailNumber::new();
        // copy left
        self.numbers.iter().for_each(|l| {
            new_snail.numbers.push(l.add(rhs.numbers.len() > 0));
        });
        // append copy right
        rhs.numbers.iter().for_each(|r| {
            new_snail.numbers.push(r.add(self.numbers.len() > 0));
        });

        new_snail.reduce();

        return new_snail;
    }
}

fn largest_pair_magnitude(numbers: Vec<SnailNumber>) -> u32 {
    let mut max_magnitude: BinaryHeap<u32> = BinaryHeap::new();

    for i in 0..numbers.len() {
        for j in i+1..numbers.len() {
            if i != j {
                let first: SnailNumber = numbers[i].clone() + numbers[j].clone();
                let second: SnailNumber = numbers[i].clone() + numbers[j].clone();
                max_magnitude.push(first.magnitude());
                max_magnitude.push(second.magnitude());
            }
        }
    }
    return max_magnitude.pop().unwrap();
}

fn main() {
    let start = Instant::now();
    let stdin = io::stdin();
    let mut lines: Vec<String> = stdin.lock().lines().flatten().collect();

    let numbers: Vec<SnailNumber> = lines.iter_mut().map(|line| {
            let mut num = SnailNumber::new();
            num.parse(line);
            num.reduce();
            return num
        }).collect();
    
    let mut sum: SnailNumber = SnailNumber::new();
    for idx in 0..numbers.len() {
        sum = sum + numbers[idx].clone();
    }

    println!("Part 1\r\n{}", "-".repeat(10));
    println!("Magnitude: {}\r\n", sum.magnitude());

    println!("Part 2\r\n{}", "-".repeat(10));
    println!("Largest magnitude: {}\r\n", largest_pair_magnitude(numbers));

    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
}

