use std::collections::HashSet;

fn main() {
    println!("{}", first_non_goldbach());
}

fn first_non_goldbach() -> i32 {
    let limit = 5_000_000usize;

    let primes = sieve_of_eratosthenes(limit);

    let mut n = 9;
    loop {
        if primes.contains(&n) {
            n += 2;
            continue;
        }

        if !is_goldbach(n, &primes) {
            break;
        }

        n += 2;
    }

    n
}

fn is_goldbach(n: i32, primes: &HashSet<i32>) -> bool {
    for p in primes {
        let y = n - p;

        if y % 2 == 0 && is_square(y / 2) {
            return true;
        }
    }

    false
}

fn is_square(y: i32) -> bool {
    let sqrt = f64::sqrt(y as f64);

    return (sqrt - (sqrt as i32) as f64) == 0.0
}

fn sieve_of_eratosthenes(limit: usize) -> HashSet<i32> {
    let mut sieve = vec![true; limit + 1];

    sieve[0] = false;
    sieve[1] = false;

    for i in (4..limit).step_by(2) {
        sieve[i] = false;
    }

    let mut i: i64 = 3;

    while i * i <= limit as i64 {
        if sieve[i as usize] {
            for j in ((i * i)..(limit as i64)).step_by(i as usize * 2) {
                sieve[j as usize] = false;
            }
        }

        i += 2;
    }

    let mut results = HashSet::new();
    for i in 0..=limit {
        if sieve[i] {
            results.insert(i as i32);
        }
    }

    results
}