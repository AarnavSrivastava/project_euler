fn main() {
    println!("{}", substring_divisibility(9));
}

fn substring_divisibility(max_digit: i64) -> i64 {
    let mut digits = Vec::new();
    for i in 0..=max_digit {
        digits.push(i);
    }

    let primes = sieve_of_eratosthenes(17);

    let mut permutations = Vec::new();
    let n = digits.len();
    pandigital_permutations(&mut permutations, &mut digits, n, n);

    let mut sum = 0;
    for i in permutations {
        let x = i.len();
        let mut should_add = true;
        // substrs to check: (1, 3), (2, 5), (3, 6), (4, 7), (5, 8), (6, 9), (7, 10)
        for j in 1..=(x - 3) {
            let mut num = 0;
            for k in j..(j + 3) {
                num *= 10;
                num += i[k];
            }

            if num % primes[j - 1] != 0 {
                should_add = false;
                break;
            }
        }

        if should_add {
            let mut construction = 0;
            for j in &i {
                construction *= 10;
                construction += j;
            }

            sum += construction;
        }
    }

    sum
}

fn sieve_of_eratosthenes(limit: usize) -> Vec<i64> {
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
                sieve[j] = false
            }
        }

        i += 2;
    }

    let mut results = Vec::new();

    for i in 0..=limit {
        if sieve[i] {
            results.push(i as i64);
        }
    }

    results
}

fn pandigital_permutations(output: &mut Vec<Vec<i64>>, digits: &mut Vec<i64>, size: usize, n: usize) {
    if size == 1 {
        let mut num = 0;
        for i in &mut *digits {
            num *= 10;
            num += *i;
        }

        if num.to_string().len() == digits.len() {
            output.push((*digits).clone());
        }

        return;
    }

    for i in 0..size {
        pandigital_permutations(output, digits, size - 1, n);

        if size % 2 == 1 {
            digits.swap(0, size - 1);
        } else {
            digits.swap(i, size - 1);
        }
    }
}