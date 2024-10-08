use std::collections::VecDeque;

fn main() {
    println!("{:?}", prime_permutations(2, 9999, 1000));
}

fn prime_permutations(iter_sequence: usize, digit_num: usize, cutoff: usize) -> Vec<i32> {
    let mut primes = VecDeque::from(sieve_of_eratosthenes(digit_num, cutoff));

    let mut sequences: Vec<Vec<i32>> = Vec::new();

    while sequences.len() < iter_sequence && primes.len() > 0 {
        let base: i32 = primes.pop_front().unwrap();

        for i in &primes {
            let dif = *i - base;
            let third = dif + *i;
            if is_permutation(base, *i) && is_permutation(base, third) && primes.contains(&third) {
                sequences.push(vec![base, *i, third]);
            }
        }
    }

    println!("{:?}", sequences);

    sequences.pop().unwrap()
}

fn is_permutation(a: i32, b: i32) -> bool {
    let mut a_vec = number_to_vec(a);
    let mut b_vec = number_to_vec(b);

    a_vec.sort();
    b_vec.sort();

    a_vec == b_vec
}

fn number_to_vec(n: i32) -> Vec<i32> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);
    digits.reverse();
    digits
}

fn sieve_of_eratosthenes(limit: usize, cutoff: usize) -> Vec<i32> {
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
            results.push(i as i32);
        }
    }
    
    results
}