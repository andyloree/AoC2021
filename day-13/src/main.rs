use std::io::{self, BufRead};
use std::time::{Instant};
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: u16,
    y: u16
}


#[derive(Debug)]
struct Fold {
    direction: char,
    value: u16
}




fn display_points(points: &Vec<Point>, rows: u16, cols: u16) {
    for col in 0..cols {
        for row in 0..rows {
            let idx = points.iter().position(|p| p.x == row && p.y == col);
            print!("{}", if idx.is_some() {"#"} else { "."});
        }
        println!();
    }

}

fn fold_along(fold: &Fold, points: &mut Vec<Point>, rows: &mut u16, cols: &mut u16) {
    points.iter_mut()
            .filter(|point| if fold.direction == 'x' { point.x >= fold.value} else { point.y >= fold.value })
            .for_each(|point| {
                if fold.direction == 'x' {
                    point.x = (fold.value * 2) - point.x;
                }
                else {
                    point.y = (fold.value * 2) - point.y;
                }
            });

    // remove
    points.sort();
    points.dedup();

    if fold.direction == 'x' {
        *rows = fold.value;
    }
    else {
        *cols = fold.value;
    }
}

fn main() {
    let start = Instant::now();
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().flatten().collect();
    let mut points: Vec<Point> = lines.iter().filter_map(|line| {
                               match line.contains(",") {
                                   false => None,
                                   true => {
                                        let mut cord = line.split(",");
                                        let x = cord.next().unwrap().parse::<u16>().unwrap();
                                        let y = cord.next().unwrap().parse::<u16>().unwrap();
                                        return Some(Point { x: x, y: y});
                                   }
                               }
                            }).collect();
                            
    let mut rows = points.iter().map(|point| point.x).max().unwrap() + 1;
    let mut cols = points.iter().map(|point| point.y).max().unwrap() + 1;

    let mut folds: Vec<Fold> = lines.iter().filter_map(|line| {
            match line.contains("fold along ") {
                false => None,
                true => {
                        let trimmed = line.replacen("fold along ","",1);
                        let mut cord = trimmed.split("=");
                        let direction = cord.next().unwrap().chars().nth(0).unwrap();
                        let value = cord.next().unwrap().parse::<u16>().unwrap();
                        return Some(Fold { direction: direction, value: value});
                }
            }
            }).collect();


    fold_along(&folds[0], &mut points, &mut rows, &mut cols);
    
    println!("Part 1\r\n{}", "-".repeat(10));
    println!("Number of points: {}\r\n", points.len());

    for i in 1..folds.len() {
        fold_along(&folds[i], &mut points, &mut rows, &mut cols);
    }
    display_points(&points, rows, cols);

    println!("Part 2\r\n{}", "-".repeat(10));
    // todo

    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
}