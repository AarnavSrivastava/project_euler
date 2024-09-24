use std::collections::HashSet;

fn main() {
    println!("{}", sum_distinct_powers(2, 100));
}

fn sum_distinct_powers(a: i64, b: i64) -> i64 {
    let mut distinct_powers = vec![(b - a) + 1; ((b - a) + 1) as usize];

    let mut possible_bases = sieve_of_eratosthenes(b as usize);

    let mut base_exponent_pairs: HashSet<(i64, i64)> = HashSet::new();

    let mut i = 2;
    while i <= b {
        if possible_bases.contains(&i) {
            for j in a..=b {
                base_exponent_pairs.insert((i, j));
            }
        } else {
            let mut base = 0;

            for x in &possible_bases {
                if i % x == 0 && is_power(*x, i) {
                    base = *x;

                    break;
                }
            }

            if base != 0 {
                for x in a..=b {
                    let base_pair = calc_pairs(base, i, x);
                    if base_exponent_pairs.contains(&base_pair) {
                        distinct_powers[(i - a) as usize] -= 1;
                    }
    
                    base_exponent_pairs.insert(base_pair);
                }
            } else {
                possible_bases.push(i);
                for j in a..=b {
                    base_exponent_pairs.insert((i, j));
                }
            }
        }
        
        i += 1;
    }

    let mut sum = 0;
    
    let mut j = 2;
    for i in distinct_powers {
        // println!("Distinct primes of {}: {}", j, i);
        sum += i;
        j += 1;
    }

    // should be 9187
    sum
}

fn calc_pairs(base: i64, num: i64, mut exponent: i64) -> (i64, i64) {
    let mut divisor = 1;
    let mut temp = base;

    while temp < num {
        divisor += 1;
        temp *= base;
    }

    exponent *= divisor;

    (base, exponent)
}

fn is_power(x: i64, i: i64) -> bool {
    let mut power = x;
    while power < i {
        power *= x;
    }

    power == i
}

fn sieve_of_eratosthenes(limit: usize) -> Vec<i64> {
    // init array
    let mut sieve = vec![true; limit + 1];

    // set 0 and 1 as false
    sieve[0] = false;
    sieve[1] = false;

    // mark all evens as false
    for i in (4..limit + 1).step_by(2) {
        sieve[i] = false;
    }

    // start a i = 3, and step by 2 since we don't have to check even values
    // only have to check numbers up to square root of n because if there is a number larger than its square root
    // that the limit is divisible by, there will be a smaller number below the square root that it is divisible by

    let mut i = 3;
    while i * i <= limit + 1 {
        if sieve[i] {
            // mark all multiples except for the number itself as false
            // start from i * i since all numbers below are already checked
            // step by i * 2 to skip even numbers
            for j in ((i * i)..(limit + 1)).step_by(i * 2) {
                sieve[j] = false
            }
        }

        // do not need to check even nums so step by two
        i += 2;
    }

    let mut results = vec![];

    for i in 0..sieve.len() {
        if sieve[i] {
            results.push(i as i64);
        }
    }

    results
}