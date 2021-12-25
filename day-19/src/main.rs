use std::io::{self, BufRead};
use std::time::{Instant};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct DistanceMatch {
    num_matched: usize,
    a_Point_idx: usize,
    b_Point_idx: usize,
    b_rotation: usize
}

impl Ord for DistanceMatch {
    fn cmp(&self, other: &DistanceMatch) -> Ordering {
        self.num_matched.cmp(&other.num_matched)
            .then_with(|| self.a_Point_idx.cmp(&other.a_Point_idx))
            .then_with(|| self.b_Point_idx.cmp(&other.b_Point_idx))
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
    becons: Vec<Point>,
    location: Point
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32
}

impl Point {
    fn new(line: &String) -> Self {
        let s = line.split(",").collect::<Vec<_>>();
        let mut iter = s.iter();
        let x = iter.next().unwrap().parse::<i32>().unwrap();
        let y = iter.next().unwrap().parse::<i32>().unwrap();
        let z = iter.next().unwrap().parse::<i32>().unwrap();
        return Point {x: x, y: y, z: z};
    }

    fn add(self, v: &Point) -> Point {
        return Point{ x: self.x + v.x, y: self.y + v.y, z: self.z + v.z };
    }

    fn subtract(self, v: &Point) -> Point {
        return Point{ x: self.x - v.x, y: self.y - v.y, z: self.z - v.z };
    }

    fn rotate(&self, rotation: usize) -> Point {
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
        return Point{ x: rotated.0, y: rotated.1, z: rotated.2 }
    }

    fn manhattan_distance(&self, other: &Point) -> i32 {
        return i32::abs(self.x - other.x) + i32::abs(self.y - other.y) + i32::abs(self.z - other.z);
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        self.x.cmp(&other.x)
            .then_with(|| self.y.cmp(&other.y))
            .then_with(|| self.z.cmp(&other.z))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl Scanner {
    fn new(lines: Vec<String>) -> Self {
        let mut scanner = Scanner{name: lines[0].to_string(), becons: vec!(), location: Point{x:0,y:0,z:0}};

        for line in &lines[1..] {
            scanner.becons.push(Point::new(line));
        }
        return scanner
    }

    fn rotate_and_locate(&mut self, rotation: usize, location: Point) {
        self.location = location;
        for i in 0..self.becons.len() {
            self.becons[i] = self.becons[i].rotate(rotation).add(&location);
        }
    }

    fn matching_sets(&self, other: &Scanner, threshold: usize) -> Option<(usize, Point, Vec<DistanceMatch>)> {
        let mut matching_becons: Vec<DistanceMatch> = vec!();
        // compare every Point relative to the other scanners Points
        // find the largest matching distance set
        for i in 0..self.becons.len() {
            let target_distances = self.distances_from(i);
            for j in 0..other.becons.len() {
                let other_distances = other.distances_from(j);
                let num_matched = intersection_count_sorted_vec(&target_distances, &other_distances) + 1; //assume two points we picked match
                if num_matched >= threshold {
                    matching_becons.push(DistanceMatch{num_matched: num_matched, a_Point_idx: i, b_Point_idx: j, b_rotation: 0});
                }

            }
        }

        // find correct rotation based upon offset of all matching pairs
        // until all of them are equal
        let mut distance_map: HashMap<Point,u32> = HashMap::new();
        for rotation in 0..24 {
            'pairs: for pair in &matching_becons {
                distance_map.entry(self.becons[pair.a_Point_idx].subtract(&other.becons[pair.b_Point_idx].rotate(rotation)))
                            .and_modify(|num_same| *num_same += 1).or_insert(1);
                if distance_map.len() > 1 {
                    break 'pairs;  // all must be equadistant
                }
            }
            if distance_map.len() == 1 {
                // we found our correct rotation, rotate all other points by the rotation
                // since we start with zero, everything will eventually end up rotated
                // correctly relative to zero
                for i in 0..matching_becons.len() {
                    matching_becons[i].b_rotation = rotation;
                }
                // Figure out position of other scanner relative to use
                let other_location = self.becons[matching_becons[0].a_Point_idx]
                        .subtract(&other.becons[matching_becons[0].b_Point_idx].rotate(rotation));

                return Some((rotation, other_location, matching_becons));
            }
            else 
            {
                // maybe next rotation
                distance_map.clear();
            }
        }
        
        return None
    }
    
    fn distances_from(&self, idx: usize) -> Vec<u64> {
        let mut distances: Vec<u64> = self.becons.iter().enumerate().filter(|&(i,_)| i != idx).map(|(_,v)| {
            euclid_distance(&self.becons[idx], &v)
        }).collect::<Vec<u64>>();
        distances.sort();
        return distances;
    }

    fn merge(&mut self, other_becons: &Vec<Point>){
        for v in other_becons {
            if !self.becons.contains(v) {
                self.becons.push(v.clone());
            }
        }
    }
}


fn euclid_distance(u: &Point, v: &Point) -> u64 {
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

fn normalize(scanners: &mut Vec<Scanner>) {
    let mut work_queue: Vec<usize> = vec!();
    let mut finished: HashMap<usize,bool> = HashMap::new();
    finished.insert(0, true);
    work_queue.push(0);
    
    while let Some(a) = work_queue.pop() {
        for b in (0..scanners.len()).filter(|&b| b != a && !finished.contains_key(&b)).collect::<Vec<usize>>() {
            match &scanners[a].matching_sets(&scanners[b], 12) {
                None => (),
                Some((rotation, b_location, pairs)) => {
                    &scanners[b].rotate_and_locate(*rotation, *b_location);
                    // add all of becons to scanner, since we know their relative location
                    let b_becons = scanners[b].becons.clone();
                    &scanners[0].merge(&(b_becons));
                    println!("Scanner {}, location {}", b, b_location);
                    work_queue.push(b);
                    finished.insert(b, true);
                } 
            }
        }
    }
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

    normalize(&mut scanners);

    println!("Part 1\r\n{}", "-".repeat(10));
    println!("Number of becons: {}", scanners[0].becons.len());

    let mut scanner_distances: Vec<i32> = vec!();
    for a in 0..scanners.len() {
        for b in a+1..scanners.len() {
            scanner_distances.push(scanners[a].location.manhattan_distance(&scanners[b].location));        }
    }

    println!("Part 2\r\n{}", "-".repeat(10));
    println!("Max distance: {}", scanner_distances.into_iter().max().unwrap());

    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
}
