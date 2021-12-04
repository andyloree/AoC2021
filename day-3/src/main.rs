    use std::io::{self, BufRead};
    
    #[derive(Eq, PartialEq)]
    enum Rating {
        O2,
        CO2
    }
    
    fn main() {
        let stdin = io::stdin();
        let diag: Vec<i32> = stdin.lock().lines().flatten().flat_map(|bstr| i32::from_str_radix(&bstr, 2)).collect();
        
        let bit_len: usize = 32 - diag.iter().max().unwrap().leading_zeros() as usize;
    
        let half_reports = (diag.len() / 2) as i32;
        let mut bit_counts: Vec<i32> = vec![0; bit_len];
        diag.iter().for_each(|val| {
            let mut i = 0;
            let mut shifted_val = val.clone();
            while shifted_val > 0 {
                bit_counts[i] += shifted_val & 1;
                shifted_val = shifted_val >> 1;
                i += 1;
            }
        });
    
        let gamma: i32 = bit_counts.iter().enumerate().map(|(i,ones)| ((ones >= &half_reports) as i32) << i).sum();
        let epsilon : i32 = bit_counts.iter().enumerate().map(|(i,ones)| ((ones < &half_reports) as i32) << i).sum();
        
        println!("Part 1\r\n{}", "-".repeat(10));
        println!("Gamma: {}\tEpsilon: {}\tPower Consumption: {}\r\n", gamma, epsilon, gamma * epsilon);
    
    
        let o2_rating = find_rating(&diag, Rating::O2, bit_len).unwrap();
        let co2_rating = find_rating(&diag, Rating::CO2, bit_len).unwrap();
    
        println!("Part 2\r\n{}", "-".repeat(10));
        println!("O2 Rating: {}\tCO2 Rating: {}\tLife Support Rating: {}", o2_rating, co2_rating, o2_rating * co2_rating);
    }
    
    fn find_rating(diag: &Vec<i32>, which_rating: Rating, bit_len: usize ) -> Option<i32> {
        let mut i: usize = 1;
        let mut criteria: i32 = 0;
        let mut filter_diag = diag.clone();
        loop {
            // Build our o2_bit_mask
            let set_size = filter_diag.len();
            let num_set: i32 = filter_diag.iter().filter(|val| (*val & (1 << bit_len - i)) > 0).count() as i32;
            if (which_rating == Rating::O2 && num_set >= (set_size as i32) - num_set) ||
               (which_rating == Rating::CO2 && num_set < (set_size as i32) - num_set) {
                criteria = criteria | 1 << (bit_len - i);
            }
            // Further filter down our set
            filter_diag = filter_diag.iter().filter(|&val| (val >> (bit_len - i)) ==  criteria >> (bit_len - i)).cloned().collect();
            // Found single match
            if filter_diag.len() == 1 {
                return Some(*filter_diag.first().unwrap());
            }
            // No more bits to match
            if bit_len - i == 0 {
                return None
            }
            i += 1;
        }
    }