use std::io::{BufRead, Write, stdout, stdin};
use std::time::{Instant};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use crossterm::{
    ExecutableCommand, QueueableCommand,
    style::{self, Attribute, Color, Print},
    Result,
    event,
};

#[derive(Copy, Clone, Eq, PartialEq)]
struct MinVertex {
    v: usize,
    dist: usize
}

impl Ord for MinVertex {
    fn cmp(&self, other: &MinVertex) -> Ordering {
        other.dist.cmp(&self.dist)
            .then_with(|| self.v.cmp(&other.v))
    }
}

impl PartialOrd for MinVertex {
    fn partial_cmp(&self, other: &MinVertex) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn expanded_map_from_input(lines: &Vec<String>) -> (Vec<u8>, usize, usize) {
    let (mut map,rows,cols) = map_from_input(lines);

    let mut new_map: Vec<u8> = vec![0;rows*5 * cols*5];

    for row in 0..rows {
        for col in 0..cols {
            new_map[col + row * cols * 5] = map[col + row * cols];
            new_map[col + cols * 1 + row * cols * 5] = if map[col + row * cols] + 1 > 9 { map[col + row * cols] + 1 - 9} else { map[col + row * cols] + 1 };
            new_map[col + cols * 2 + row * cols * 5] = if map[col + row * cols] + 2 > 9 { map[col + row * cols] + 2 - 9} else { map[col + row * cols] + 2 };
            new_map[col + cols * 3 + row * cols * 5] = if map[col + row * cols] + 3 > 9 { map[col + row * cols] + 3 - 9} else { map[col + row * cols] + 3 };
            new_map[col + cols * 4 + row * cols * 5] = if map[col + row * cols] + 4 > 9 { map[col + row * cols] + 4 - 9} else { map[col + row * cols] + 4 };
        }
    }

    for offset in 0..4 {
        for idx in (rows * cols * 5 * offset)..(rows * cols * 5 * (offset + 1)) {
            new_map[idx + rows * cols * 5 * 1] = if new_map[idx] + 1 > 9 { new_map[idx] + 1 - 9 } else { new_map[idx] + 1 };
        }
    }

    return (new_map, rows * 5, cols * 5);
}

fn map_from_input(lines: &Vec<String>) -> (Vec<u8>, usize, usize) {
    let rows = lines.len();
    let cols = lines[0].len();
    let mut map: Vec<u8> = vec![0; rows * cols];

    lines.into_iter().enumerate().for_each(|(row,line)| {
        line.chars().enumerate().for_each(|(col,c)| {
            map[col + row * cols] = c.to_string().parse::<u8>().unwrap();
        });
    });

    return (map, rows, cols);
}

fn print_map(map: &Vec<u8>, rows: usize, cols: usize, path: Option<&Vec<usize>>) {
    let mut stdout = stdout();
    for row in 0..rows {
        for col in 0..cols {
            if path.is_some() && path.unwrap().contains(&(row * cols + col)) {
                stdout.queue(style::SetBackgroundColor(Color::DarkYellow));
            }
            else {
                stdout.queue(style::SetBackgroundColor(Color::Black));
            }
            stdout.queue(style::Print(map[col + row * cols]));
        }
        stdout.queue(style::SetBackgroundColor(Color::Black));
        stdout.queue(style::Print("\n\r"));
    }
    stdout.flush();
}


fn vertex_neighbor(v: usize, rows: usize, cols: usize) -> Vec<usize> {
    let shift: [i32;4] = [-(cols as i32), -1, 1, cols as i32];
    let mut neighbors: Vec<usize> = vec!();
    let col = v % cols;
    let row = (v - col) / rows;

    for offset in shift {
        if !((row == 0 && offset < -1 ) ||
             (row == rows - 1 && offset > 1 ) ||
             (col == 0 && offset == -1 ) ||
             (col == cols - 1 && offset == 1)) {
                neighbors.push((v as i32 + offset) as usize);
        }
    }
    return neighbors;
   
}

fn shortest_risk_path(map: &Vec<u8>, rows: usize, cols: usize) -> Option<(usize,Vec<usize>)> {
    let start: usize = 0;
    let end: usize = cols * rows - 1;

    // initial "distance" to all other nodes is max/unknown
    let mut dist: Vec<usize> = vec![usize::MAX; map.len()];
    let mut prev: Vec<usize> = vec![usize::MAX; map.len()];
    let mut pqueue: BinaryHeap<MinVertex> = BinaryHeap::new();

    dist[start] = 0;
    pqueue.push(MinVertex { v: start, dist: 0});

    // Find the next lowest cost node (priority queue)
    while let Some( MinVertex {v: u, dist: u_dist}) = pqueue.pop() {
        if u == end {
            // reconstruct our path
            let mut path: Vec<usize> = vec!();
            let mut cur_v = end;

            while prev[cur_v] != usize::MAX {
                path.push(cur_v);
                cur_v = prev[cur_v];
            }
            path.push(start);

            return Some((u_dist, path));
        }
        for v in vertex_neighbor(u, rows, cols) {
            let alt = dist[u] + map[v] as usize;
            if alt < dist[v] {
                dist[v] = alt;
                prev[v] = u;
                pqueue.push(MinVertex { v: v, dist: u_dist + map[v] as usize});
            }
        }
    }
    return None;
}


fn main() {
    let start = Instant::now();
    let stdin = stdin();
    let lines: Vec<String> = stdin.lock().lines().flatten().collect();

    println!("Part 1\r\n{}", "-".repeat(10));
    let (mut map,rows,cols) = map_from_input(&lines);
    if let Some((shortest,path)) = shortest_risk_path(&map,rows,cols) {
        print_map(&map,rows,cols,Some(&path));
        println!("Shortest path: {}\r\n", shortest);
    }


    println!("Part 2\r\n{}", "-".repeat(10));
    let (mut map,rows,cols) = expanded_map_from_input(&lines);
    if let Some((shortest,path)) = shortest_risk_path(&map,rows,cols) {
        println!("Shortest path: {}\r\n", shortest);
    }

    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
}