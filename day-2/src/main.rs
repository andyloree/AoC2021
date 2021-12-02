use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().flatten().collect();

    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    lines.iter().for_each(|line| {
        let mut command = line.split_whitespace();
        let direction = command.next().unwrap();
        let magnitude = command.next().unwrap().parse::<i32>().unwrap();
        match direction {
            "forward" => horizontal += magnitude,
            "up" => depth -= magnitude,
            "down" => depth += magnitude,
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
        let magnitude = command.next().unwrap().parse::<i32>().unwrap();
        match direction {
            "forward" => {horizontal += magnitude;  depth += aim * magnitude},
            "up" => aim -= magnitude,
            "down" => aim += magnitude,
            _ =>  println!("Throw the switch Vern, she's pumping mud")
        }
    });

    println!("Part 2\r\n{}", "-".repeat(10));
    println!("Horizontal: {}\tDepth: {}\tProduct: {}\r\n", horizontal, depth, horizontal * depth);

}
