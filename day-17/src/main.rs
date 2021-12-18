use std::io::{self, BufRead};
use std::time::{Instant};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug)]
struct Target {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32
}

#[derive(Debug,Eq, PartialEq, Copy, Clone)]
struct Probe {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    cur_dx: i32,
    cur_dy: i32,
    max_y: i32
}

impl Ord for Probe {
    fn cmp(&self, other: &Probe) -> Ordering {
        self.max_y.cmp(&other.max_y)
            .then_with(|| other.x.cmp(&self.x))
            .then_with(|| other.y.cmp(&self.y))
    }
}

impl PartialOrd for Probe {
    fn partial_cmp(&self, other: &Probe) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Probe {
    fn new(dx: i32, dy: i32) -> Self {
        Probe {
            x: 0,
            y: 0,
            dx: dx,
            dy: dy,
            cur_dx: dx,
            cur_dy: dy,
            max_y: 0
        }
    }

    fn step(&mut self) {
        self.x += self.cur_dx;
        self.y += self.cur_dy;
        self.max_y = i32::max(self.y, self.max_y);

        if self.cur_dx > 0 { self.cur_dx -= 1} else if self.cur_dx < 0 { self.cur_dx += 1}
        self.cur_dy -= 1;
    }
}

impl std::fmt::Display for Probe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "dx {}, dy: {}", self.dx, self.dy)
    }
}

impl Target {
    fn new(line: String) -> Self {
        let clean = line.replacen("target area: ", "", 1).replacen("x=","",1).replacen(" y=","",1).replacen("..",",",2);
        let cordinates: Vec<i32> = clean.split(",").map(|c| c.parse::<i32>().unwrap()).collect();

        Target {
            x1: *cordinates[0..=1].iter().min().unwrap(),
            x2: *cordinates[0..=1].iter().max().unwrap(),
            y1: *cordinates[2..=3].iter().min().unwrap(),
            y2: *cordinates[2..=3].iter().max().unwrap()
        }
    }
    /// returns true if a given 
    fn does_it_hit(&mut self, probe: &mut Probe) -> bool {
        loop {
            probe.step();
            // too far to the right or below target
            if probe.x > self.x2 || probe.y < self.y1 {
                return false;            
            }

            // hit
            if probe.x >= self.x1 && probe.x <= self.x2 &&
               probe.y >= self.y1 && probe.y <= self.y2 {
                return true;
            }
        }
    }
}


fn main() {
    let start = Instant::now();
    let stdin = io::stdin();
    let line: String = stdin.lock().lines().nth(0).unwrap().unwrap();

    let mut max_height: BinaryHeap<Probe> = BinaryHeap::new();
    let mut target = Target::new(line);

    for x in 0..=target.x2 {
        for y in target.y1..=i32::abs(target.y1) {
            let mut probe = Probe::new(x,y);
            if target.does_it_hit(&mut probe) {
                max_height.push(probe);
            }
        }
    }

    println!("Part 1\r\n{}", "-".repeat(10));
    println!("Number of hits: {}", max_height.len());
    
    let probe = max_height.pop().unwrap();
    println!("Max height: {} - {}", probe.max_y, probe);

    println!("Part 2\r\n{}", "-".repeat(10));
    // todo

    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
}
