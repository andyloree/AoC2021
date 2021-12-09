use std::io::{self, BufRead};
use std::time::{Duration, Instant};

fn signal_to_bits(signal: &str) -> u8 {
    let mut bits: u8 = 0;
    for c in signal.chars() {
        match c {
            'a' => bits |= 0b0000001,
            'b' => bits |= 0b0000010,
            'c' => bits |= 0b0000100,
            'd' => bits |= 0b0001000,
            'e' => bits |= 0b0010000,
            'f' => bits |= 0b0100000,
            'g' => bits |= 0b1000000,
             _  => unreachable!()
        }
    }
    return bits;
}

fn count_unique_digits(line: &String, digits_counts: &mut [i32;7]) -> i32 {
    let mut wires = line.split("|");
    let signals: Vec<&str> = wires.next().unwrap().split(" ").filter(|val| val.len() > 0).collect::<Vec<&str>>();
    let digits: Vec<&str> = wires.next().unwrap().split(" ").filter(|val| val.len() > 0).collect::<Vec<&str>>();
    let mut unqiue_digits = 0;
    digits.iter().map(|digit| digit.len() as i32)
            .for_each(|len| {
                digits_counts[(len as usize) - 1] += 1;
                if len >= 2 && len <= 4 || len == 7 {
                    unqiue_digits += 1;
                }
            });
    return unqiue_digits;
}


fn map_segments_to_output(line: &String) -> i32 {
    let mut wires = line.split("|");
    let signals: Vec<&str> = wires.next().unwrap().split(" ").filter(|val| val.len() > 0).collect::<Vec<&str>>();
    let digits: Vec<&str> = wires.next().unwrap().split(" ").filter(|val| val.len() > 0).collect::<Vec<&str>>();

    let mut segment_map: [u8;10] = [0;10];
    let unique_signals: Vec<&str> = signals.iter().filter(|signal| signal.len() >= 2 && signal.len() <= 4 || signal.len() == 7).map(|s| s.as_ref()).collect();
    let other_signals: Vec<&str> = signals.iter().filter(|signal| !(signal.len() >= 2 && signal.len() <= 4 || signal.len() == 7)).map(|s| s.as_ref()).collect();
    // Unique signals
    for signal in unique_signals {
        let bits = signal_to_bits(signal);
        match signal.len() {
            2 => segment_map[1] = bits,    // 1
            3 => segment_map[7] = bits,    // 7
            4 => segment_map[4] = bits,    // 4
            7 => segment_map[8] = bits,    // 8
            _ => unreachable!()
        }
    }

    for signal in other_signals {
        let bits = signal_to_bits(signal);
        match signal.len() {
            5 => { // 2|3
                if (bits & segment_map[4]).count_ones() == 2 {
                    segment_map[2] = bits;  // 2
                }
                else if (bits & segment_map[1]).count_ones() == 2 ||
                          (bits & segment_map[7]).count_ones() == 3 {
                    segment_map[3] = bits;  // 3
                }
                else {
                    segment_map[5] = bits;  // 5
                }
            },
            6 => { 
                if (bits & segment_map[7]).count_ones() == 2 ||
                   (bits & segment_map[1]).count_ones() == 1 {
                    segment_map[6] = bits;  // 6
                }
                else if (bits & segment_map[4]).count_ones() == 4 {
                    segment_map[9] = bits;  // 9
                }
                else {
                    segment_map[0] = bits;  // 0
                }
            },
            _ => unreachable!()
        }
    }
    assert_eq!(segment_map.into_iter().filter(|&has_bits| has_bits > 0).count(),10);

    // map our digits from segments, shifting digits by 10's place
    let output: i32 = digits.iter().map(|digit| signal_to_bits(digit)).enumerate().map(|(idx,bits)| {
        (segment_map.iter().position(|&value| value == bits).unwrap() as i32) * i32::pow(10, 3 - idx as u32)
    }).sum::<i32>();
    return output;
}


fn main() {
    let start = Instant::now();
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().flatten().collect();

    let mut counts: [i32;7] = [0;7];
    let mut unique_total: i32 = 0;
    let mut output_total: i32 = 0;
    lines.into_iter().for_each(|line| {
        unique_total += count_unique_digits(&line, &mut counts);
        output_total += map_segments_to_output(&line);
    });

    println!("Part 1\r\n{}", "-".repeat(10));
    println!("Unique Counts: {:?}", counts);
    println!("Total: {:?}\r\n", unique_total);

    println!("Part 2\r\n{}", "-".repeat(10));
    println!("Output: {:?}\r\n", output_total);

    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
}