use std::io::{self, BufRead};

fn main() {
    // Read from stdin to vec
    let stdin = io::stdin();
    let mut depths: Vec<i32> = Vec::new();
    for line in stdin.lock().lines() {
        depths.push(line.unwrap().parse::<i32>().unwrap());
    }
    
    // part 1
    println!("Part 1\r\n{}", "-".repeat(10));
    num_increases(&depths);
    
    // part 2
    println!("Part 2\r\n{}", "-".repeat(10));
    let summed_depths = summed_sliding_values(&depths, 3);
    num_increases(&summed_depths);
}

fn num_increases(depths: &Vec<i32> ) {
    let mut num_increases: i32 = 0;
    let mut prev_depth: i32 = -1;
    
    for cur_depth in depths {
        if prev_depth > 0 && *cur_depth > prev_depth {
            num_increases += 1;
        }
        prev_depth = *cur_depth;
    }

    println!("Number of increases: {}\r\n", num_increases);
}

fn summed_sliding_values(depths: &Vec<i32>, window: usize) -> Vec<i32> {
    let mut summed_depths:  Vec<i32> = Vec::new();
    
    let mut count = 0;
    for sample in depths.windows(window) {
        summed_depths.push(sample.iter().sum());
        count += 1;
    }

    return summed_depths;
}
