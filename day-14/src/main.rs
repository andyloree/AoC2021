use std::io::{self, BufRead};
use std::time::{Instant};
use std::collections::HashMap;

fn rule_to_pairs(base_pair: &String, insert: String) -> Vec<String> {
    let mut pairs: Vec<String> = vec!();
    pairs.push(base_pair.to_string());

    for (i, letter) in base_pair.chars().enumerate().collect::<Vec<(_,_)>>() {
        let mut split: String = String::new();
        if i == 0 {
             split.push_str(letter.to_string().as_str());
             split.push_str(insert.as_str());
        }
        else {
            split.push_str(insert.as_str());
            split.push_str(letter.to_string().as_str());
        }
        pairs.push(split);
    }
    return pairs;
}

fn create_map_from_rules(lines: &Vec<String>) -> (HashMap<String, u64>, HashMap<String,(String,String)>) {

    let mut polymers: HashMap<String, u64> = HashMap::new();
    let mut rules: HashMap<String,(String,String)> = HashMap::new();
    lines.iter().enumerate().filter(|&(i,_)| i > 1).for_each(|(_,line)| {
        // for slice in line.chars().collect::<Vec<_>>().windows(2) {
        //     println!("{:?}", slice);
        // }


        // for letter in base_pair.chars().collect::<Vec<_>>() {

        // }
        let mut rule = line.split(" -> ");
        let base_pair = rule.next().unwrap().to_string();
        let insert = rule.next().unwrap().to_string();
        let pairs = rule_to_pairs(&base_pair,insert);

        // add to our polymer map counts
        for pair in pairs.iter() {
            if !polymers.contains_key(pair) {
                polymers.insert(pair.to_string(), 0);
            }
        }

        // rules recipe
        rules.insert(base_pair, (pairs[1].to_string(), pairs[2].to_string()));
    });

    return (polymers, rules);
}

fn populate_from_template(template: String, polymers: &mut HashMap<String, u64>) {
    for slice in template.chars().collect::<Vec<_>>().windows(2) {
        let pair: String = slice.iter().collect();
        *polymers.get_mut(&pair).unwrap() += 1;
    }
}

fn score_polymers(polymers: HashMap<String, u64>, last_letter: String) -> u64 {
    let mut occurrances: HashMap<String,u64> = HashMap::new();
    polymers.keys().for_each(|base_pair| {
        let letter = base_pair.chars().nth(0).unwrap();
        let num_pairs = polymers.get(base_pair).unwrap();

        *occurrances.entry(letter.to_string()).or_insert(0) += num_pairs;
    });

    *occurrances.entry(last_letter).or_insert(0) += 1;

    let min_letter_count = occurrances.values().min().unwrap();
    let max_letter_count = occurrances.values().max().unwrap();

    return max_letter_count - min_letter_count;
}

fn polymer_iterations(template: &String, lines: &Vec<String>, iterations: usize) -> u64 {
    let (mut polymers, rules) = create_map_from_rules(&lines);
    populate_from_template(template.to_string(), &mut polymers);

    
    for _n in 0..iterations {
        let mut tally: HashMap<String,u64> = HashMap::new();
        polymers.keys().for_each(|base_pair| {
            let (split_1,split_2) = rules.get(base_pair).unwrap();
            let num_pairs = polymers.get(base_pair).unwrap();

            *tally.entry(split_1.to_string()).or_insert(0) += num_pairs;
            *tally.entry(split_2.to_string()).or_insert(0) += num_pairs;
        });
        
        polymers.clear();
        tally.keys().for_each(|base_pair| {
            polymers.insert(base_pair.to_string(), *tally.get(base_pair).unwrap());
        });
    }

    return score_polymers(polymers, template.chars().last().unwrap().to_string());
}


fn main() {
    let start = Instant::now();
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().flatten().collect();

    let template: &String = lines.iter().nth(0).unwrap();
    let score = polymer_iterations(template, &lines, 10);

    println!("Part 1\r\n{}", "-".repeat(10));
    println!("Score: {}\r\n", polymer_iterations(template, &lines, 10));

  
    println!("Part 2\r\n{}", "-".repeat(10));
    println!("Score: {}", polymer_iterations(template, &lines, 40));

    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
}