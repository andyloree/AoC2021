use std::io::{self, BufRead};
use std::time::{Instant};
use std::collections::HashMap;

struct Vertice {
    name: String,
    is_big: bool,
    is_start: bool,
    is_end: bool,
}

fn vertice_from_name(name: &String) -> Vertice {
    return Vertice {
        name: String::from(name),
        is_big: *name == name.to_uppercase(),
        is_start: name.to_lowercase() == "start",
        is_end: name.to_lowercase() == "end",
    };
}

fn parse_line(line: String,v: &mut Vec<Vertice>,adj: &mut Vec<Vec<usize>>) {
    let names: Vec<&str> = line.split("-").collect();

    // add vertice or edge
    let indexes: Vec<usize> = names.into_iter().map(|name| {
         match v.iter().position(|node| node.name == name) {
            Some(idx) => return idx,
            None => {
                // New vertice
                v.push(vertice_from_name(&String::from(name)));
                adj.push(vec!());
                return v.len() - 1;
            }
        }
    }).collect();

    let idx1 = indexes.iter().nth(0).unwrap();
    let idx2 = indexes.iter().nth(1).unwrap();

    adj[*idx1].push(*idx2);
    adj[*idx2].push(*idx1);

}

fn get_all_walks(v: Vec<Vertice>,adj: Vec<Vec<usize>>, double_small_visit: bool) -> Vec<String> {
    let mut path_list: Vec<String> = vec!();
    let mut walks: HashMap<String,bool> = HashMap::new();
    let mut visits_left: Vec<u8> = vec![1; v.len()];
    let start = v.iter().position(|node| node.name == "start").unwrap();
    let end = v.iter().position(|node| node.name == "end").unwrap();
    path_list.push("start".to_string());

    if double_small_visit {
        let smalls: Vec<usize> = v.iter().enumerate().filter(|(_i,node)| !node.is_big && !node.is_start && !node.is_end).map(|(i,node)| i).collect();

        for small_idx in smalls {
            visits_left = vec![1; v.len()];
            visits_left[small_idx] += 1;
            recurse_walks(start, end, &v, &adj, &mut visits_left, &mut path_list, &mut walks);
        }
    }
    else {
        recurse_walks(start, end, &v, &adj, &mut visits_left, &mut path_list, &mut walks);
    }

    return walks.keys().cloned().collect();
}

fn recurse_walks(u: usize, d: usize,v: &Vec<Vertice>,adj: &Vec<Vec<usize>>,visits_left: &mut Vec<u8>, path_list: &mut Vec<String>,  walks: &mut HashMap<String,bool>) {
    if u == d {
        let this_walk = path_list.join(",");
        if !walks.contains_key(&this_walk) {
            walks.insert(path_list.join(","), true);
        }
        // we found the end, back up
        return;
    }

    // mark small nodes as visited, big can backtrack as many times as they want
    if !v[u].is_big {
        if visits_left[u] == 0 {
            return;
        }
        visits_left[u] -=  1;
    }
    
    let c: Vec<usize> = (adj[u]).iter().filter(|x| visits_left[**x] > 0).map(|&x| x).collect();
    c.iter().for_each(|x| {
        path_list.push(v[*x].name.to_string());
        recurse_walks(*x, d, &v, &adj, visits_left,path_list, walks);
        path_list.remove(path_list.len() - 1);
    });

    if !v[u].is_big {
        visits_left[u] += 1;
    }
}

fn main() {
    let start = Instant::now();
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().flatten().collect();

    let mut v: Vec<Vertice> = vec!();
    let mut adj: Vec<Vec<usize>> = vec!();
    lines.iter().for_each(|line| {
        parse_line(line.to_string(), &mut v, &mut adj);
    });

    let walks = get_all_walks(v,adj, false);

    println!("Part 1\r\n{}", "-".repeat(10));
    println!("Number of walks: {}", walks.len());

    let mut v: Vec<Vertice> = vec!();
    let mut adj: Vec<Vec<usize>> = vec!();
    lines.iter().for_each(|line| {
        parse_line(line.to_string(), &mut v, &mut adj);
    });
    let walks = get_all_walks(v,adj, true);
    println!("Part 2\r\n{}", "-".repeat(10));
    println!("Number of walks with two smalls: {}", walks.len());

    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
}