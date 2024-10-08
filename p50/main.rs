use std::collections::VecDeque;

fn main() {
    println!("{}", consecutive_prime_sum(1_000_000));
}

fn consecutive_prime_sum(limit: usize) -> i64 {
    let mut primes = VecDeque::from(sieve_of_eratosthenes(limit, 2));

    let mut max_sum = primes[0];
    let mut max_seq = vec![primes[0]];

    while !primes.is_empty() {
        let mut curr_sum = primes.pop_front().unwrap();
        let mut curr_seq = vec![curr_sum];

        for i in &primes {
            curr_sum += i;
            curr_seq.push(*i);

            if curr_sum > limit as i64 {
                break;
            }

            if curr_sum > max_sum && curr_seq.len() >= max_seq.len() && primes.contains(&curr_sum) {
                max_sum = curr_sum;
                max_seq = curr_seq.clone();
            }
        }
    }

    max_sum
}

fn sieve_of_eratosthenes(limit: usize, cutoff: usize) -> Vec<i64> {
    let mut sieve = vec![true; limit + 1];

    sieve[0] = false;
    sieve[1] = false;

    for i in (4..=limit).step_by(2) {
        sieve[i] = false;
    }

    let mut i = 3;
    while i * i <= limit {
        if sieve[i] {
            for j in ((i * i)..=limit).step_by(i * 2) {
                sieve[j] = false;
            }
        }

        i += 2;
    }

    let mut results = Vec::new();
    for i in cutoff..=limit {
        if sieve[i] {
            results.push(i as i64);
        }
    }
    
    results
}