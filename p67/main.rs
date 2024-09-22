use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let rows = read_file("/Users/aarnavsrivastava/Desktop/_/rust_problems/p67/triangle.txt")?;

    println!("{}", max_path_sum(rows)?);

    Ok(())
}

fn max_path_sum(rows: Vec<Vec<i32>>) -> Result<i32, Box<dyn Error>> {
    let mut max_sum = 0;
    let mut sums: Vec<Vec<i32>> = Vec::new();

    // to keep track of:
    // indices adjacent to current "j" value
    // sums from each path

    sums.push(rows[0].clone());

    for i in 1..rows.len() {
        sums.push(Vec::new());
        
        for j in 0..rows[i].len() {
            if j == 0 {
                let prev_val_one = sums[i - 1][0];

                sums[i].push(rows[i][j] + prev_val_one);

                continue;
            }

            if j == rows[i].len() - 1 {
                let prev_index = sums[i - 1].len() - 1;
                let prev_val_two = sums[i - 1][prev_index];

                sums[i].push(rows[i][j] + prev_val_two);

                continue;
            }

            let prev_val_one = sums[i - 1][j];
            let prev_val_two = sums[i - 1][j - 1];

            sums[i].push(std::cmp::max(prev_val_one + rows[i][j], prev_val_two + rows[i][j]));
        }
    }
    
    for i in &sums[sums.len() - 1] {
        if *i > max_sum {
            max_sum = *i;
        }
    }

    Ok(max_sum)
}

fn read_file(file_path: &str) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let mut rows = Vec::new();

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut row = Vec::new();

        for k in line?.split(" ") {
            row.push(k.parse::<i32>().unwrap());
        }

        rows.push(row);
    }

    Ok(rows)
}