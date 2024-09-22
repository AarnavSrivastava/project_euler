use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let grid = gen_grid("/Users/aarnavsrivastava/Desktop/_/rust_problems/p11/input.txt", 20)?;

    println!("{}", largest_product_in_grid(grid, 4));

    Ok(())
}

fn largest_product_in_grid(grid: Vec<Vec<i32>>, till: usize) -> i64 {
    let mut max_prod: i64 = 0;

    // 1. check horizontal
    for i in 0..grid.len() {
        for j in 0..(grid[i].len() - till) {
            let mut prod = 1;

            for k in j..(j + till) {
                prod *= grid[i][k] as i64;
            }

            if prod > max_prod {
                max_prod = prod;
            }
        }
    }

    // 2. check vertical
    for i in 0..(grid.len() - till) {
        for j in 0..grid[i].len() {
            let mut prod = 1;

            for k in i..(i + till) {
                prod *= grid[k][j] as i64;
            }

            if prod > max_prod {
                max_prod = prod;
            }
        }
    }

    // 3. check diagonal forward
    for i in 0..(grid.len() - till) {
        for j in 0..(grid.len() - till) {
            let mut prod = 1;

            for k in 0..till {
                prod *= grid[i + k][j + k] as i64;
            }

            if prod > max_prod {
                max_prod = prod;
            }
        }
    }

    // 4. check diagonal reverse
    for i in till..grid.len() - till {
        for j in 0..(grid.len() - till) {
            let mut prod = 1;

            for k in 0..till {
                prod *= grid[i - k][j + k] as i64;
            }

            if prod > max_prod {
                max_prod = prod;
            }
        }
    }

    max_prod
}

fn gen_grid(file_path: &str, dim: usize) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut grid = vec![vec![0; dim]; dim];

    let mut i = 0;
    for line in reader.lines() {
        let mut j = 0;
        for num in line?.split(" ") {
            grid[i][j] = num.parse::<i32>().unwrap();

            j += 1;
        }

        i += 1;
    }

    Ok(grid)
}