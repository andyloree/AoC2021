use std::io::{self, BufRead, Write, stdout};
use std::time::{Instant};
use std::thread::{sleep_ms};
use std::collections::{HashMap, VecDeque};
use std::env;
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize, Color}, Result
};
use structopt::StructOpt;


/// Commandline arguments
#[derive(StructOpt)]
struct Cli {
    visualize: Option<u32>,
    delay: Option<u32>,
    refresh: Option<u32>
}

fn mark_seen(seen: &mut HashMap<(usize, usize),char>,x: usize,y: usize, basin_num: Option<usize>) {
    match basin_num {
        None => seen.insert((x,y),'-'),
        Some(num) => seen.insert((x,y),num.to_string().chars().next().unwrap())
    };
}

fn not_seen(seen: &mut HashMap<(usize, usize),char>,x: usize,y: usize) -> bool {
    return !(seen.contains_key(&(x,y)));
}

fn render_map(map: &Vec<Vec<char>>, seen: &mut HashMap<(usize, usize),char>, delay: u32) {
    let mut stdout = stdout();
    let get_level = |x: usize,y: usize| -> i32 { map[x][y].to_string().parse::<i32>().unwrap() };
    let width:  usize = map[0].len();
    let height: usize = map.len();
    for x in 0..height {
        for y in 0..width {
            let level = get_level(x,y);
            let seen_level = seen.get(&(x,y));
            let show: String = if seen_level.is_some() || level == 9 { level.to_string() } else { ' '.to_string() };
            if seen_level.is_some() || level == 9 {
                let (r,g,b,br,bg,bb) = match level {
                    9 => (0,178,0,0,255,0),
                    0 => (204,184,184,255,230,230),
                    1 => (204,163,163,255,204,204),
                    2 => (204,143,143,255,179,179),
                    3 => (204,122,122,255,153,153),
                    4 => (204,102,102,255,128,128),
                    5 => (204,81,81,255,102,102),
                    6 => (204,61,61,255,77,77),
                    7 => (204,40,40,255,51,51),
                    8 => (204,19,19,255,25,25),
                    _ => (255,255,255,0,0,0),
                };
                stdout.queue(style::SetForegroundColor(Color::Rgb{ r: r, g: g, b: b}));
                stdout.queue(style::SetBackgroundColor(Color::Rgb{ r: br, g: bg, b: bb}));
            }
            else {
                // unseen so far
                stdout.queue(style::SetForegroundColor(Color::White));
                stdout.queue(style::SetBackgroundColor(Color::Black));
            }
            stdout.queue(cursor::MoveTo(y as u16,x as u16));
            stdout.queue(style::Print(show));
        }
        // end of line black
        stdout.queue(style::SetForegroundColor(Color::White));
        stdout.queue(style::SetBackgroundColor(Color::Black));
        stdout.queue(style::Print(" "));
    }
    stdout.flush();
    sleep_ms(delay as u32);
}

fn main() {
    let args = Cli::from_args();

    let visualize = args.visualize.is_some() && args.visualize.unwrap() != 0;
    let delay: u32 = if args.delay.is_some() { args.delay.unwrap() } else { 0 };
    let refresh: u32 = if args.refresh.is_some() { args.refresh.unwrap() } else { 0 };


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
    let mut basin_points: Vec<i32> = vec!();
    let get_level = |x: usize,y: usize| -> i32 { map[x][y].to_string().parse::<i32>().unwrap() };

    // from low points flood-fill
    let mut fill: VecDeque<(usize, usize,usize)> = VecDeque::new();
    low_points.into_iter().enumerate().for_each(|(basin_num,(x,y,_level))| {
        fill.push_back((x,y,basin_num));
        basin_points.push(0);
    });

    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All));
    let mut counter = 0;
    while fill.len() > 0 {
        let (x, y, basin_num) = fill.pop_front().unwrap();
        if not_seen(&mut seen,x,y) {
            basin_points[basin_num as usize] += 1; // add to current basin
            mark_seen(&mut seen,x,y, Some(basin_num)); // mark it seen

            if visualize && (counter >= refresh || fill.len() == 0) {
                render_map(&map, &mut seen, delay);
                counter = 0;
            }
            else {
                counter += 1;
            }

            // start walking in all directions
            if x > 0 && not_seen(&mut seen,x - 1,y) && get_level(x - 1,y) != 9 {
                fill.push_back((x - 1,y,basin_num)); // queue walk north
            }
            if x < height - 1 && not_seen(&mut seen,x + 1,y) && get_level(x + 1,y) != 9 {
                fill.push_back((x + 1,y,basin_num)); // queue walk south
            }
            if y > 0 && not_seen(&mut seen,x,y - 1) && get_level(x,y - 1) != 9 {
                fill.push_back((x,y - 1,basin_num)); // queue walk west
            }
            if y < width - 1 && not_seen(&mut seen,x,y + 1) && get_level(x,y + 1) != 9 {
                fill.push_back((x,y + 1,basin_num)); // queue walk east
            }

        }
    }

    basin_points.sort();
    basin_points.reverse();

    if !visualize {
        println!("\r\nPart 1\r\n{}", "-".repeat(10));
        println!("Risk Level: {:?}\r\n", risk_level);

        println!("Part 2\r\n{}", "-".repeat(10));
        println!("Basins points: {:?}\r\n", &basin_points[0..=2]);
        println!("Basins: {:?}\r\n", &basin_points[0..=2].iter().fold(1,|acc,point| { acc * point }));

        let duration = start.elapsed();
        println!("Total execution time: {:?}", duration);
    }
    else {
        println!();
    }
}