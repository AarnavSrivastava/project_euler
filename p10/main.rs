fn main() {
    println!("{:?}", sum_of_primes(2_000_000));
}

fn sum_of_primes(limit: i64) -> i64 {
    let mut sum: i64 = 0;

    let sieve = sieve_of_eratosthenes(limit);

    for i in 0..sieve.len() {
        if sieve[i] {
            sum += i as i64;
        }
    }

    sum
}

fn sieve_of_eratosthenes(limit: i64) -> Vec<bool> {
    // init array
    let mut sieve = vec![true; (limit + 1) as usize];

    // set 0 and 1 as false
    sieve[0] = false;
    sieve[1] = false;

    // mark all evens as false
    for i in (4..limit + 1).step_by(2) {
        sieve[i as usize] = false;
    }

    // start a i = 3, and step by 2 since we don't have to check even values
    // only have to check numbers up to square root of n because if there is a number larger than its square root
    // that the limit is divisible by, there will be a smaller number below the square root that it is divisible by

    let mut i = 3;
    while i * i <= limit + 1 {
        if sieve[i as usize] {
            // mark all multiples except for the number itself as false
            // start from i * i since all numbers below are already checked
            // step by i * 2 to skip even numbers
            for j in ((i * i)..(limit + 1)).step_by((i * 2) as usize) {
                sieve[j as usize] = false
            }
        }

        // do not need to check even nums so step by two
        i += 2;
    }

    sieve
}