use std::io::{self, BufRead};
use std::collections::HashMap;
use std::time::{Duration, Instant};


fn main() {
    let start = Instant::now();
    let stdin = io::stdin();
    let mut numbers: Vec<i32> = stdin.lock().lines().next().unwrap().unwrap().split(",").flat_map(|s| s.trim().parse::<i32>()).collect();

    // median
    numbers.sort();
    let median = numbers[numbers.len() / 2];
    let min_sum_distance = numbers.iter().fold(0,|acc,val| acc + i32::abs(val - median));
    
    println!("Part 1\r\n{}", "-".repeat(10));
    print!("Median: {}\r\n", median);
    print!("Min Sum Distance: {}\r\n", min_sum_distance);


    // nearest two points about the means using triangular summation
    let mean = numbers.iter().sum::<i32>() as f32 / numbers.len() as f32;
    let mut nearest_targets: Vec<i32> = vec![mean.floor() as i32];
    if mean.fract() != 0.0 { nearest_targets.push(mean.ceil() as i32)}
    let mut min_target: i32 = mean.floor() as i32;
    let mut min_sum_distance: i32 = std::i32::MAX;
    for i in nearest_targets  {
        let cur = numbers.iter().fold(0,|acc,val| acc + ((i32::abs(val - i) * i32::abs(val - i) + i32::abs(val - i)) / 2) );
        if cur < min_sum_distance {
            min_target = i;
            min_sum_distance = cur;
        }
    }

    println!("Part 2\r\n{}", "-".repeat(10));
    print!("Target: {}\r\n", min_target);
    print!("Min Sum Distance: {}\r\n", min_sum_distance);

    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
}