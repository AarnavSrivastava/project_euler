fn main() {
    println!("{}", max_reciprocal_factor(1000));
}

fn max_reciprocal_factor(limit: usize) -> usize {
    let mut max_cycle = 0;

    let sieve = sieve_of_eratosthenes(limit);

    for i in sieve {
        let mut cycle = 1;
        let mut j = 1;

        while {
            j = j * 10 % i;

            cycle += 1;
            

            j > 1
        } { }

        if cycle > max_cycle {
            max_cycle = cycle;
        }
    }

    max_cycle
}

// sieve to find prime numbers
fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut sieve = vec![true; limit + 1];

    sieve[0] = false;
    sieve[1] = false;

    for i in (4..limit + 1).step_by(2) {
        sieve[i] = false;
    }

    let mut i = 1;
    while i * i <= limit + 1 {
        if sieve[i] {
            for j in ((i * i)..=limit).step_by(i * 2) {
                sieve[j] = false;
            }
        }

        i += 2;
    }

    let mut results = vec![];

    for i in 0..sieve.len() {
        if sieve[i] {
            results.push(i);
        }
    }

    results
}