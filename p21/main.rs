fn main() {
    use std::time::Instant;
    let now = Instant::now();

    println!("{}", calc_sum_of_amicable_numbers(10000));

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn calc_sum_of_amicable_numbers(limit: usize) -> usize {
    let mut sum = 0;

    let mut sieve = vec![false; limit + 1];

    for i in 1..=limit {
        if sieve[i] {
            continue;
        }

        // calc sum of divisors for i; this is effectively another integer "j"
        let sum_divisors = calc_sum_divisors(i);

        // check that sum_divisors falls within the limit and that sum_divisors does not equal "i"
        if sum_divisors > limit || sum_divisors == i {
            continue;
        }

        // calc sum of divisors for "j"
        let other_divisors = calc_sum_divisors(sum_divisors);

        // compare sum of divisors for "j" to "i"
        if other_divisors == i {
            sieve[i] = true;
            sieve[sum_divisors] = true;

            sum += i + sum_divisors;
        }
    }

    sum
}

fn calc_sum_divisors(num: usize) -> usize {
    let mut sum = 1;

    let mut i = 2;
    while i * i <= num {
        if num % i == 0 {
            sum += i;
            
            if num / i != i {
                sum += num / i;
            }
        }

        i += 1;
    }

    sum
}