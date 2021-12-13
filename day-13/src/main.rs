use std::io::{self, BufRead};
use std::time::{Instant};

#[derive(Debug)]
struct Point {
    x: u16,
    y: u16
}

fn main() {
    let start = Instant::now();
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().flatten().collect();
    let points: Vec<Point> = lines.iter().filter_map(|line| {
                               match line.contains(",") {
                                   false => None,
                                   true => {
                                        let mut cord = line.split(",");
                                        let x = cord.next().unwrap().parse::<u16>().unwrap();
                                        let y = cord.next().unwrap().parse::<u16>().unwrap();
                                        return Some(Point { x: x, y: y});
                                   }
                               }
                            }).collect();
                            
    println!("{:?}", points);
    println!("Part 1\r\n{}", "-".repeat(10));
    // todo

    println!("Part 2\r\n{}", "-".repeat(10));
    // todo

    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
}