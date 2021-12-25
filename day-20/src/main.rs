use std::io::{self, BufRead};
use std::time::{Instant};

struct Image {
    rows: usize,
    cols: usize,
    image: Vec<bool>
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut str: String = String::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                str.push_str(if self.image[col + row * self.cols] { "#" } else { "." });
            }
            str.push_str("\r\n");
        }
        write!(f, "{}", str)
    }
}

impl Image {
    fn new(rows: usize, cols: usize) -> Self {
        return Image{rows: rows, cols: cols, image: vec!()};
    }

    fn expand(&self, void: bool) -> Self {
        let mut new_image = Image{rows: self.rows + 2, cols: self.cols + 2, image: vec![false;(self.cols + 2) * (self.rows + 2)]};

        self.image.iter().enumerate().for_each(|(center,bit)| {
            let col = center % self.cols;
            let row = ((center as i32 - col as i32) / self.rows as i32) as usize;
            let dest_center = (col + 1) + (row + 1) * new_image.cols;
            new_image.image[dest_center] = *bit;
        });

        // blink infinte state
        for idx in 0..new_image.cols {
            new_image.image[idx] = void;
        }
        for idx in ((new_image.rows - 1) * new_image.cols)..new_image.image.len() {
            new_image.image[idx] = void;
        }
        for idx in (0..(new_image.rows * new_image.cols)).step_by(new_image.cols) {
            new_image.image[idx] = void;
        }
        for idx in (new_image.cols - 1..(new_image.rows * new_image.cols)).step_by(new_image.cols) {
            new_image.image[idx] = void;
        }

        return new_image;
    }

    fn pixle_reading(&self, center: usize, default: bool) -> usize {
        let shift: [i32;9] = [-(self.cols as i32) - 1, -(self.cols as i32), 1 - (self.cols as i32),
                               -1, 0, 1,
                               (self.cols as i32) - 1, (self.cols as i32), (self.cols as i32) + 1];

        let col = center % self.cols;
        let row = (center - col) / self.rows;
        let mut csi_index: usize = 0;

        for (bit, &offset) in shift.iter().enumerate() {
            if !((row == 0 && offset < -1 ) ||              // no rows above
                (row == self.rows - 1 && offset > 1 ) ||    // no rows below
                (col == 0 && (offset == -1 || offset == -(self.cols as i32) - 1 || offset == (self.cols as i32) - 1 )) || // left side 
                (col == self.cols - 1 && (offset == 1 || offset ==  1 - (self.cols as i32) || offset == (self.cols as i32) + 1 ))) {
                    csi_index |= (self.image[(center as i32 + offset) as usize] as usize) << 8 - bit;
            }
            else {
                csi_index |= (default as usize) << 8 - bit
            }   
        }
        return csi_index;
    }

    fn enhance_image(&self, algorithm: &Vec<bool>, generation: usize) -> Image {
        let default = if generation % 2 != 0 { algorithm[0] } else { false };
        let mut new_image = self.expand(default);
        let mut enhanced: Vec<bool> = vec![false;new_image.image.len()];
        for center in 0..new_image.image.len() {
            let csi_index = new_image.pixle_reading(center,default);
            // todo fix shift of center
            enhanced[center] = algorithm[csi_index];
        }
        new_image.image = enhanced;
        return new_image;
    }

    fn num_lit_pixels(&self) -> usize {
        return self.image.iter().filter(|bit| **bit).count();
    }
}

fn image_from_string(lines: Vec<String>) -> Image {
    let rows = lines.len();
    let cols = lines[0].len();
    let mut image: Vec<bool> = vec!();
    lines.iter().for_each(|line| {
        line.chars().for_each(|c| {
            image.push(c == '#');
        });
    });
    return Image{rows,cols,image};
}

fn main() {
    let start = Instant::now();
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().flatten().collect();

    let algorithm: Vec<bool> = lines[0].chars().map(|c| c == '#').collect();
    let mut image  = image_from_string(lines[2..].to_vec());

    for generation in 0..50 {
        image = image.enhance_image(&algorithm,generation);
        if generation == 1 {
            println!("Part 1\r\n{}", "-".repeat(10));
            println!("Lit pixel count:\r\n{}", image.num_lit_pixels());
        }
    }

    println!("Part 2\r\n{}", "-".repeat(10));
    println!("Lit pixel count:\r\n{}", image.num_lit_pixels());
  
    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
}