use std::io::{self, BufRead, Write, stdout};
use std::time::{Instant};
use std::collections::HashMap;
use std::thread::{sleep_ms};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize, Color}, Result
};
use structopt::StructOpt;

// Commandline arguments
#[derive(StructOpt)]
struct Cli {
    visualize: Option<u32>,
    delay: Option<u32>
}

fn visualize_grid(octopuses: [u8; 100], delay: u32) {
    let mut stdout = stdout();

    let draw_point = &mut |r: u8, g: u8, b: u8, val:u8, col: usize, row: usize | {
        stdout.queue(style::SetBackgroundColor(Color::Rgb{ r: r, g: g, b: b}));

        for row_mux in 0..4 {
            stdout.queue(cursor::MoveTo((col) as u16 * 6 as u16,(((row + 1) * 4) - row_mux) as u16));
            stdout.queue(style::Print("      ")); //val.to_string().repeat(6)));
        }
    };
    
    for col in 0..10 {
        for row in 0..10 {
            let val = octopuses[row + col * 10];
            let (r,g,b) = match octopuses[row + col * 10] {
                9 => (221,160,221),
                0 => (0,0,0),
                1 => (16,16,16),
                2 => (32,32,32),
                3 => (48,48,48),
                4 => (64,64,64),
                5 => (80,80,80),
                6 => (96,96,96),
                7 => (112,112,112),
                8 => (128,128,128),
                _ => (255,255,255),
            };
            draw_point(r,g,b,val, col,row);
        }
    }
    
    stdout.queue(cursor::MoveTo(60 as u16,40 as u16));
    stdout.queue(style::SetBackgroundColor(Color::Black));
    stdout.flush();
    sleep_ms(delay as u32);
}

fn bloom(center: usize) -> Vec<usize> {
    let shift: [i32;8] = [-11, -10 , -9, -1, 1, 9, 10 ,11];
    let mut indexes: Vec<usize> = vec!();
    let col = center % 10;
    let row = (center - col) / 10;
    for offset in 0..8 {
        if !((row == 0 && offset <= 2 ) ||
             (row == 9 && offset >= 5 ) ||
             (col == 0 && (offset == 0 || offset == 3 || offset == 5) ) ||
             (col == 9 && (offset == 2 || offset== 4 || offset == 7))) {
            indexes.push((center as i32 + shift[offset]) as usize);
        }
    }
    return indexes;
}

fn step(octopuses: &mut [u8; 100], visualize: bool, delay: u32) -> i32 {
    let mut flash: Vec<usize> = vec!();
    let mut counter: i32 = 0;
    for idx in 0..100 {
        if octopuses[idx] < 9 {
            octopuses[idx] += 1;
        }
        else {
            flash.push(idx);
            octopuses[idx] = 0;
            counter += 1;
        }
    }

    while flash.len() > 0 {
        // apply energy to surrounding octos
        let center = flash.pop().unwrap();
        for idx in bloom(center) {
            if octopuses[idx] > 0 && octopuses[idx] < 9 {
                octopuses[idx] += 1;
            }
            else if octopuses[idx] == 9  {
                flash.push(idx);
                octopuses[idx] = 0;
                counter += 1;
            }
        }
        if visualize { visualize_grid(*octopuses, delay) };
        
    }
    if visualize { visualize_grid(*octopuses, delay) };

    return counter;
}

fn main() {
    let args = Cli::from_args();

    let visualize = args.visualize.is_some() && args.visualize.unwrap() != 0;
    let delay: u32 = if args.delay.is_some() { args.delay.unwrap() } else { 0 };

    let start = Instant::now();
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().flatten().collect();

    let mut octopuses: [u8; 100] = [0; 100];
    lines.iter().enumerate().for_each(|(row,line)| {
        line.chars().enumerate().for_each(|(col,c)| {
            octopuses[col + row * 10] = c.to_digit(10).unwrap() as u8;
        });
    });

    let mut total_flashes_100 = 0;
    let mut cur_step = 0;
    loop {
        let flashes = step(&mut octopuses, visualize, delay);
        if cur_step < 100 { total_flashes_100 += flashes; }
        if octopuses.into_iter().map(|val| val as i32).sum::<i32>() == 0 {
            break;
        }
        cur_step += 1;
    }
    if !visualize {
        println!("\r\nPart 1\r\n{}", "-".repeat(10));
        println!("Total flashes: {}\r\n", total_flashes_100);


        println!("Part 2\r\n{}", "-".repeat(10));
        println!("All flashes: {}\r\n", cur_step + 1);

        let duration = start.elapsed();
        println!("Total execution time: {:?}", duration);
    }
}