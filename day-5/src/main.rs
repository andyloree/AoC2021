use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::LinkedList;
use std::time::{Duration, Instant};
use std::cmp::{min,max};

fn add_line_to_map(line: String, points_map: &mut HashMap<(i16,i16),i16>, overlap_points: &mut LinkedList<(i16,i16)>, fourty_fives: bool) {
    const X1: usize = 0;const Y1: usize = 1;const X2: usize = 2;const Y2: usize = 3;
    let mut ends: [i16;4] = [0; 4];
    line.split(" -> ").flat_map(|points| points.split(",").flat_map(|point| point.parse::<i16>()))
                            .enumerate().for_each(|(idx,coor)| ends[idx] = coor);

    // add point closure
    let mut add_point = |point: (i16,i16)| {
        if points_map.contains_key(&point) {
            let num_overlaps = points_map.remove(&point).unwrap();
            if num_overlaps == 1 {
                overlap_points.push_back(point);
            }
            points_map.insert(point, num_overlaps + 1);
        }
        else {
            points_map.insert(point, 1);    // first point
        }
    };

    //let mut points: Vec<(i16,i16)> = vec!();
    if ends[X1] == ends[X2] {
        // horizontal
        for y in min(ends[Y1], ends[Y2])..(max(ends[Y1], ends[Y2]) + 1) {
            add_point((ends[X1], y));
        }
    }
    else if ends[Y1] == ends[Y2] {
        // horizontal
        for x in min(ends[X1], ends[X2])..(max(ends[X1], ends[X2]) + 1) {
            add_point((x, ends[Y1]));
        }
    }
    else if fourty_fives {
        // absolute slope of one
        if ((ends[Y2] - ends[Y1]) as f32 / (ends[X2] - ends[X1]) as f32).abs() == 1f32 {
            // change in x/y as we walk the line and add points
            let dx = if ends[X2] - ends[X1] > 0 { 1 } else { -1 };
            let dy = if ends[Y2] - ends[Y1] > 0 { 1 } else { -1 };
            let mut cur_point = (ends[X1], ends[Y1]);
            
            while cur_point != (ends[X2], ends[Y2]) {
                add_point(cur_point);
                cur_point = (cur_point.0 + dx, cur_point.1 + dy);
            }
            add_point((ends[X2],ends[Y2]));
        }
        else {
            return;
        }
    }
    else {
        return;
    }
}

fn main() {
    let start = Instant::now();
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().flatten().collect();
    
    let mut overlap_points: LinkedList<(i16,i16)> = LinkedList::new();
    let mut points_map: HashMap<(i16,i16),i16> = HashMap::new();
    lines.iter().for_each(|line| {
        add_line_to_map(line.to_string(), &mut points_map, &mut overlap_points, false);
    });

    println!("Part 1\r\n{}", "-".repeat(10));
    println!("# overlap points: {}\r\n", overlap_points.len());

    let mut overlap_points: LinkedList<(i16,i16)> = LinkedList::new();
    let mut points_map: HashMap<(i16,i16),i16> = HashMap::new();
    lines.into_iter().for_each(|line| {
        add_line_to_map(line, &mut points_map, &mut overlap_points, true);
    });

    println!("Part 2\r\n{}", "-".repeat(10));
    println!("# overlap points: {}\r\n", overlap_points.len());

    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
}
