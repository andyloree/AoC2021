use std::io::{self, BufRead};
use std::time::{Instant};
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct DistanceMatch {
    num_matched: usize,
    a_beacon_idx: usize,
    b_beacon_idx: usize,
    b_rotation: usize
}

impl Ord for DistanceMatch {
    fn cmp(&self, other: &DistanceMatch) -> Ordering {
        self.num_matched.cmp(&other.num_matched)
            .then_with(|| self.a_beacon_idx.cmp(&other.a_beacon_idx))
            .then_with(|| self.b_beacon_idx.cmp(&other.b_beacon_idx))
            .then_with(|| self.b_rotation.cmp(&other.b_rotation))
    }
}

impl PartialOrd for DistanceMatch {
    fn partial_cmp(&self, other: &DistanceMatch) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug)]
struct Scanner {
    name: String,
    beacons: Vec<Beacon>
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Beacon {
    x: i32,
    y: i32,
    z: i32
}

impl Beacon {
    fn new(line: &String) -> Self {
        let s = line.split(",").collect::<Vec<_>>();
        let mut iter = s.iter();
        let x = iter.next().unwrap().parse::<i32>().unwrap();
        let y = iter.next().unwrap().parse::<i32>().unwrap();
        let z = iter.next().unwrap().parse::<i32>().unwrap();
        return Beacon {x: x, y: y, z: z};
    }

    fn add(self, v: &Beacon) -> Beacon {
        return Beacon{ x: self.x + v.x, y: self.y + v.y, z: self.z + v.z };
    }

    fn subtract(self, v: &Beacon) -> Beacon {
        return Beacon{ x: self.x - v.x, y: self.y - v.y, z: self.z - v.z };
    }

    fn rotate(&self, rotation: usize) -> Beacon {
        let rotated = match rotation {
            0  => ( self.x,  self.y,  self.z),
            1  => ( self.x,  self.z, -self.y),
            2  => ( self.x, -self.y, -self.z),
            3  => ( self.x, -self.z,  self.y),
            4  => ( self.y,  self.x, -self.z),
            5  => ( self.y,  self.z,  self.x),
            6  => ( self.y, -self.x,  self.z),
            7  => ( self.y, -self.z, -self.x),
            8  => ( self.z,  self.x,  self.y),
            9  => ( self.z,  self.y, -self.x),
            10 => ( self.z, -self.x, -self.y),
            11 => ( self.z, -self.y,  self.x),
            12 => (-self.x,  self.y, -self.z),
            13 => (-self.x,  self.z,  self.y),
            14 => (-self.x, -self.y,  self.z),
            15 => (-self.x, -self.z, -self.y),
            16 => (-self.y,  self.x,  self.z),
            17 => (-self.y,  self.z, -self.x),
            18 => (-self.y, -self.x, -self.z),
            19 => (-self.y, -self.z,  self.x),
            20 => (-self.z,  self.x, -self.y),
            21 => (-self.z,  self.y,  self.x),
            22 => (-self.z, -self.x,  self.y),
            23 => (-self.z, -self.y, -self.x),
            _ => unreachable!()
        };
        return Beacon{ x: rotated.0, y: rotated.1, z: rotated.2 }
    }
}

impl fmt::Display for Beacon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Ord for Beacon {
    fn cmp(&self, other: &Beacon) -> Ordering {
        self.x.cmp(&other.x)
            .then_with(|| self.y.cmp(&other.y))
            .then_with(|| self.z.cmp(&other.z))
    }
}

impl PartialOrd for Beacon {
    fn partial_cmp(&self, other: &Beacon) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl Scanner {
    fn new(lines: Vec<String>) -> Self {
        let mut scanner = Scanner{name: lines[0].to_string(), beacons: vec!()};

        for line in &lines[1..] {
            scanner.beacons.push(Beacon::new(line));
        }
        return scanner
    }


    fn matching_sets(&self, other: &Scanner, threshold: usize) -> Vec<DistanceMatch> {
        let mut matching_sets: Vec<DistanceMatch> = vec!();
        // compare every beacon relative to the other scanners beacons
        // find the largest matching distance set
        // as well as rotating b across all 24 axis
        for i in 0..self.beacons.len() {
            let target_distances = self.distances_from(i, 0);
            for rotation in 0..24 {
                for j in 0..other.beacons.len() {
                    let other_distances = other.distances_from(j, rotation);
                    let num_matched = intersection_count_sorted_vec(&target_distances,&other_distances);
                    if num_matched + 1 >= threshold {
                        matching_sets.push(DistanceMatch{num_matched: num_matched, a_beacon_idx: i, b_beacon_idx: j, b_rotation: rotation })
                    }
                }
            }
        }
        return matching_sets;
    }
    
    fn distances_from(&self, idx: usize, rotation: usize) -> Vec<u64> {
        let mut distances: Vec<u64> = self.beacons.iter().enumerate().filter(|&(i,_)| i != idx).map(|(_,v)| {
            euclid_distance(&self.beacons[idx], &Beacon::rotate(v, rotation))
        }).collect::<Vec<u64>>();
        distances.sort();
        return distances;
    }
}


fn euclid_distance(u: &Beacon, v: &Beacon) -> u64 {
    return ((u.x as f64 - v.x as f64).powf(2.0) + (u.y as f64 - v.y as f64).powf(2.0) + (u.z as f64 - v.z as f64).powf(2.0)) as u64;
}

fn intersection_count_sorted_vec(a: &Vec<u64>, b: &Vec<u64>) -> usize {
    let mut count = 0;
    let mut b_iter = b.iter();
    if let Some(mut current_b) = b_iter.next() {
        for current_a in a {
            while current_b < current_a {
                current_b = match b_iter.next() {
                    Some(current_b) => current_b,
                    None => return count,
                };
            }
            if current_a == current_b {
                count += 1;
            }
        }
    }
    count
}

fn relative_scanner_location(a: &Beacon, b: &Beacon) -> Beacon {
    return b.subtract(a);
    //return &a.add(&a.subtract(&b));
}

fn main() {
    let start = Instant::now();
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().flatten().collect();
    // split our input a blank line delimiters
    let scanner_lines = lines.iter().fold(vec!(), |mut acc: Vec<Vec<String>>, line| {
        let mut skip = false;
        if line.len() == 0 || acc.is_empty() {
            skip = !acc.is_empty();
            acc.push(vec!());
        }
        
        if !skip {
            acc.last_mut().unwrap().push(line.to_string());
        }
        acc
    });

    let mut scanners: Vec<Scanner> = scanner_lines.iter().map(|lines| Scanner::new(lines.to_vec())).collect();

    for a in 0..scanners.len() {
        for b in a+1..scanners.len() {
            let sets = &scanners[a].matching_sets(&scanners[b], 12);
            println!("{}", sets.len());
            for set in sets {
                //println!("Match set {} - {}, matched: {}, rotation: {}:", scanners[a].name, scanners[b].name, set.num_matched, set.b_rotation);
                let scanner_b_location = relative_scanner_location(&scanners[a].beacons[set.a_beacon_idx], &Beacon::rotate(&scanners[b].beacons[set.b_beacon_idx], set.b_rotation));
                //println!("Scanner {} relative to {}: {}", a, b, scanner_b_location);
            }
        }
    }

    println!("Part 1\r\n{}", "-".repeat(10));
    // todo

    println!("Part 2\r\n{}", "-".repeat(10));
    // todo

    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
}
