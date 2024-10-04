fn main() {
    println!("{}", pentagon_numbers());
    // println!("{}", is_pentagonal(146));
}

fn pentagon_numbers() -> i64 {
    let big_num = 100_000 as i64;
    let mut min_dif = i64::MAX;

    let mut pentagonal_numbers = Vec::new();
    for i in 1..=big_num {
        let n = (i * (3 * i - 1) / 2) as i64;
        pentagonal_numbers.push(n);

        let index = (i - 2) as usize;

        if index > pentagonal_numbers.len() {
            continue;
        }

        if n - pentagonal_numbers[index] > min_dif {
            continue;
        }

        for j in (0..(i - 1)).rev() {
            let dif = n - pentagonal_numbers[j as usize];

            if dif > min_dif {
                break;
            }

            let sum = n + pentagonal_numbers[j as usize];
            if is_pentagonal(dif) && is_pentagonal(sum) && dif < min_dif {
                min_dif = dif;
            }
        }
    }

    min_dif
}

fn is_pentagonal(n: i64) -> bool {
    let x: f64 = (1.0 + f64::sqrt((n * 24 + 1) as f64)) / 6.0;

    let pos = x as i64;

    x - pos as f64 == 0.0
}