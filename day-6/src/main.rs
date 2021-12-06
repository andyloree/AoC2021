use std::io::{self, BufRead};
use std::time::{Duration, Instant};

fn num_fish_by_days(num_days: usize, fish_by_days: &mut [i64;9]) -> i64 {
    for day in 1..num_days+1 {
        fish_by_days.rotate_left(1);
        fish_by_days[6] += fish_by_days[8];
    }
    return fish_by_days.iter().sum::<i64>();
}

fn main() {
    let start = Instant::now();
    let stdin = io::stdin();

    let mut fish_by_days: [i64;9] = [0;9];
    stdin.lock().lines().flatten().for_each(|line| {
        line.split(",").flat_map(|s| s.trim().parse::<usize>()).for_each(|fish_day| {
            fish_by_days[fish_day] += 1; 
        });
    });

    println!("Part 1\r\n{}", "-".repeat(10));
    println!("Number fish after {} days: {}\r\n", 80, num_fish_by_days(80,&mut fish_by_days));

    println!("Part 2\r\n{}", "-".repeat(10));
    println!("Number fish after {} days: {}\r\n", 256, num_fish_by_days(256 - 80,&mut fish_by_days));

    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
}
