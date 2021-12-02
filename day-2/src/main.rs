use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().flatten().collect();

    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    lines.iter().for_each(|line| {
        let mut command = line.split_whitespace();
        let direction = command.next().unwrap();
        let dim = command.next().unwrap().parse::<i32>().unwrap();
        match direction {
            "forward" => horizontal += dim,
            "up" => depth -= dim,
            "down" => depth += dim,
            _ =>  println!("Throw the switch Vern, she's pumping mud")
        }
    });

    println!("Part 1\r\n{}", "-".repeat(10));
    println!("Horizontal: {}\tDepth: {}\tProduct: {}\r\n", horizontal, depth, horizontal * depth);


    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    lines.iter().for_each(|line| {
        let mut command = line.split_whitespace();
        let direction = command.next().unwrap();
        let dim = command.next().unwrap().parse::<i32>().unwrap();
        match direction {
            "forward" => {horizontal += dim;  depth += aim * dim},
            "up" => aim -= dim,
            "down" => aim += dim,
            _ =>  println!("Throw the switch Vern, she's pumping mud")
        }
    });

    println!("Part 2\r\n{}", "-".repeat(10));
    println!("Horizontal: {}\tDepth: {}\tProduct: {}\r\n", horizontal, depth, horizontal * depth);

}
