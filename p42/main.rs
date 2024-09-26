use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let words = read_file("/Users/aarnavsrivastava/Desktop/_/project_euler/p42/words.txt")?;
    println!("{}", coded_triangle_nums(words));

    Ok(())
}

fn coded_triangle_nums(words: Vec<String>) -> i32 {
    let ASCII_LOWER: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 
        'f', 'g', 'h', 'i', 'j', 
        'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't', 
        'u', 'v', 'w', 'x', 'y', 
        'z',
    ];

    let mut count = 0;

    let tri_nums = triangle_nums(1500);

    for word in words {
        let mut sum = 0;
        for c in word.chars() {
            let index = ASCII_LOWER.iter().position(|&r| r == c).unwrap();
            sum += (index + 1) as i32;
        }

        if tri_nums.contains(&sum) {
            count += 1;
        }
    }

    count
}

fn read_file(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut words = Vec::new();

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        for word in line?.split(",") {
            words.push(word.replace("\"", "").to_lowercase());
        }
    }

    Ok(words)
}

// calc only triangle nums up to 1500
fn triangle_nums(limit: usize) -> Vec<i32> {
    let mut nums = Vec::new();

    for i in 1..=limit {
        nums.push(((i * (i + 1)) / 2) as i32);
    }

    nums
}