use std::io::{self, BufRead};

fn main() {
    // Read from stdin to vec
    let stdin = io::stdin();
    let mut depths: Vec<i32> = Vec::new();
    for line in stdin.lock().lines() {
        depths.push(line.unwrap().parse::<i32>().unwrap());
    }
    
    // Procedureal
    // part 1
    println!("Part 1\r\n{}", "-".repeat(10));
    println!("{}\r\n", num_increases(&depths));
    
    // part 2
    println!("Part 2\r\n{}", "-".repeat(10));
    let summed_depths = summed_sliding_values(&depths, 3);
    println!("{}\r\n", num_increases(&summed_depths));

    // Functional versions
    // part 1
    println!("Functional Part 1\r\n{}", "-".repeat(10));
    println!("{}\r\n", depths.windows(2).map(|value| { value[1] > value[0] } ).filter(|&increase| increase).count());

    // part 1
    println!("Functional Part 2\r\n{}", "-".repeat(10));
    println!("{}", depths.windows(3).map(|sample| { sample.iter().fold(0,|acc,x| { acc + x})}).collect::<Vec<i32>>()
                         .windows(2).map(|value| { value[1] > value[0] } ).filter(|&increase| increase).count());

}

fn num_increases(depths: &Vec<i32> ) -> i32 {
    let mut num_increases: i32 = 0;
    let mut prev_depth: i32 = -1;
    
    for cur_depth in depths {
        if prev_depth > 0 && *cur_depth > prev_depth {
            num_increases += 1;
        }
        prev_depth = *cur_depth;
    }

    return num_increases;
}

fn summed_sliding_values(depths: &Vec<i32>, window: usize) -> Vec<i32> {
    let mut summed_depths:  Vec<i32> = Vec::new();

    for sample in depths.windows(window) {
        summed_depths.push(sample.iter().sum());
    }

    return summed_depths;
}
