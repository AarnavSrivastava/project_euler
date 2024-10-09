use std::collections::HashMap;

fn main() {
    println!("{}", prime_digit_replacements(8));
}

fn prime_digit_replacements(num_digits: i32) -> usize {
    let primes = sieve_of_eratosthenes(10_000_000);

    let mut families: HashMap<String, i32> = HashMap::new();
    for p in 0..=10_000_000 {
        if !primes[p] {
            continue;
        }

        let p_string = p.to_string();

        // println!("{}", p);

        let mut potential_families = Vec::new();
        for i in 0..=9 {
            let replaced = p_string.replace(&i.to_string(), "n");
            
            if replaced != *p_string {
                potential_families.push(replaced);
            }
        }
        
        for family in &potential_families {
            if !families.contains_key(family) {
                let mut count = 0;

                for i in 0..=9 {
                    let num_str = family.replace("n", &i.to_string());
                    let num: i32 = num_str.parse().unwrap();
                    
                    if num % 2 != 0 && num.to_string().len() == num_str.len() && primes[num as usize] {
                        count += 1;
                        families.insert((*family).clone(), count);
                    }
                }

                if count >= num_digits {
                    return p;
                }
            }
        }
    }

    0
}

fn sieve_of_eratosthenes(limit: usize) -> Vec<bool> {
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

    sieve
}