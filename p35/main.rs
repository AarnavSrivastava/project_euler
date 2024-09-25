use std::collections::HashSet;

fn main() {
    println!("{}", count_circular_primes(1_000_000));
}

fn count_circular_primes(limit: usize) -> i32 {
    let mut found_circulars = HashSet::new();

    let primes = sieve_of_eratosthenes(limit);

    for i in 0..primes.len() {
        if found_circulars.contains(&primes[i]) {
            continue;
        }

        let mut shift = 1;
        let n = primes[i];

        while shift * 10 < n {
            shift *= 10;
        }

        let mut rotated = n;
        let mut found_nums = vec![n];
        loop {
            let digit = rotated % 10;
            rotated /= 10;

            rotated += digit * shift;

            found_nums.push(rotated);

            if !primes.contains(&rotated) {
                break;
            }

            if rotated == n {
                break;
            }
        }

        if rotated == n {
            for i in found_nums {
                found_circulars.insert(i);
            }
        }
    }

    found_circulars.len() as i32
}

fn sieve_of_eratosthenes(limit: usize) -> Vec<i32> {
    let mut sieve = vec![true; limit + 1];

    sieve[0] = false;
    sieve[1] = false;

    for i in (4..=limit).step_by(2) {
        sieve[i] = false;
    }

    let mut i = 3;
    while i * i <= limit + 1 {
        if sieve[i] {
            for j in ((i * i)..=limit).step_by(i * 2) {
                sieve[j] = false;
            }
        }

        i += 2;
    }

    let mut results = Vec::new();
    for i in 0..=limit {
        if sieve[i] {
            results.push(i as i32);
        }
    }

    results
}