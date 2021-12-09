use std::io::{self, BufRead};
use std::time::{Instant};
use std::collections::{HashMap, VecDeque};

fn mark_seen(seen: &mut HashMap<(usize, usize),char>,x: usize,y: usize, basin_num: Option<i32>) {
    match basin_num {
        None => seen.insert((x,y),'-'),
        Some(num) => seen.insert((x,y),num.to_string().chars().next().unwrap())
    };
}

fn not_seen(seen: &mut HashMap<(usize, usize),char>,x: usize,y: usize) -> bool {
    return !(seen.contains_key(&(x,y)));
}

fn main() {
    let start = Instant::now();
    let stdin = io::stdin();
    let map: Vec<Vec<char>> = stdin.lock().lines().map(|line| line.unwrap().chars().collect::<Vec<char>>()).collect();

    let width:  usize = map[0].len();
    let height: usize = map.len();
    let mut low_points: Vec<(usize,usize,i32)> = vec!();
    for x in 0..height {
        for y in 0..width {
            let north = if x == 0          { ':' } else { map[x - 1][y]};
            let south = if x == height - 1 { ':' } else { map[x + 1][y]};
            let west =  if y == 0          { ':' } else { map[x][y - 1]};
            let east =  if y == width - 1  { ':' } else { map[x][y + 1]};
            let cur = map[x][y];

            if north > cur && south > cur && east > cur && west > cur {
                low_points.push((x,y,map[x][y].to_string().parse::<i32>().unwrap()));
            }
        }
    }

    let risk_level: i32 = low_points.iter().map(|(_x,_y,level)| level + 1).sum();

    let mut seen: HashMap<(usize, usize),char> = HashMap::new();
    let mut basin_num: i32 = -1;
    let mut basin_points: Vec<i32> = vec!();

    let get_level = |x: usize,y: usize| -> i32 { map[x][y].to_string().parse::<i32>().unwrap() };

    for x in 0..height {
        for y in 0..width {
            // not been walked yet
            if not_seen(&mut seen,x,y) {
                let level = get_level(x,y);
                if level < 9 {
                    // new basin
                    basin_num += 1;
                    basin_points.push(0);

                    let mut fill: VecDeque<(usize, usize)> = VecDeque::new();
                    fill.push_front((x,y));
                    while fill.len() > 0
                    {
                        let (cur_x, cur_y) = fill.pop_front().unwrap();
                        if not_seen(&mut seen,cur_x,cur_y) {
                            basin_points[basin_num as usize] += 1; // add to current basin
                            mark_seen(&mut seen,cur_x,cur_y, Some(basin_num)); // mark it seen

                            // start walking in all directions
                            if cur_x > 0 && not_seen(&mut seen,cur_x - 1,cur_y) && get_level(cur_x - 1,cur_y) != 9 {
                                fill.push_front((cur_x - 1,cur_y)); // queue walk north
                            }
                            if cur_x < height - 1 && not_seen(&mut seen,cur_x + 1,cur_y) && get_level(cur_x + 1,cur_y) != 9 {
                                fill.push_front((cur_x + 1,cur_y)); // queue walk south
                            }
                            if cur_y > 0 && not_seen(&mut seen,cur_x,cur_y - 1) && get_level(cur_x,cur_y - 1) != 9 {
                                fill.push_front((cur_x,cur_y - 1)); // queue walk west
                            }
                            if cur_y < width - 1 && not_seen(&mut seen,cur_x,cur_y + 1) && get_level(cur_x,cur_y + 1) != 9 {
                                fill.push_front((cur_x,cur_y + 1)); // queue walk east
                            }
                        }
                    }
                }
                else
                {   // peek
                    mark_seen(&mut seen, x,y, None);
                }
            }
        }
    }

    basin_points.sort();
    basin_points.reverse();



    println!("Part 1\r\n{}", "-".repeat(10));
    println!("Risk Level: {:?}\r\n", risk_level);

    println!("Part 2\r\n{}", "-".repeat(10));
    println!("Basins: {:?}\r\n", &basin_points[0..=2].iter().fold(1,|acc,point| { acc * point }));

    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
}