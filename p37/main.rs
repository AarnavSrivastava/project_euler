fn main() {
    println!("{}", truncatable_primes());
}

fn truncatable_primes() -> i32 {
    let mut sum = 0;

    // mathematically, 1,000,000 is the limit for left-to-right and right-to-left truncatable primes
    let primes = sieve_of_eratosthenes(1_000_000);
    let n = primes.len();

    for i in 0..n {
        if primes[i] < 10 {
            continue;
        }

        let mut truncatable = true;
        let mut temp_left = primes[i];

        while temp_left > 0 {
            if !primes.contains(&(temp_left)) {
                truncatable = false;
                break;
            }

            temp_left /= 10;
        }

        if !truncatable {
            continue;
        }

        let temp_right = primes[i];
        let mut shift = 10;
        while temp_right % shift != temp_right {
            if !primes.contains(&(temp_right % shift)) {
                truncatable = false;
                break;
            }

            shift *= 10;
        }

        if truncatable {
            sum += primes[i];
        }
    }

    sum
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