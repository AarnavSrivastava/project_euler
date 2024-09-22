fn main() {
    println!("{}", non_abundant_sums());
}

fn non_abundant_sums() -> i32 {
    let mut sum: i32 = 0;
    let sieve = abundant_numbers_sieve(28123);

    for i in 0..=28123 {
        let mut isnt_sum_of_abunds = true;

        for j in 0..=(i / 2) {
            if !sieve[j] {
                continue;
            }

            if sieve[j] && sieve[i - j] {
                isnt_sum_of_abunds = false;
                break;
            }
        }

        if isnt_sum_of_abunds {
            sum += i as i32;
        }
    }

    sum
}

fn abundant_numbers_sieve(limit: usize) -> Vec<bool> {
    let mut sieve = vec![false; limit + 1];

    for i in 0..=limit {
        sieve[i as usize] = is_abundant(i);
    }

    sieve
}

fn is_abundant(num: usize) -> bool {
    let mut sum = 1;

    let mut i = 2;
    while i * i <= num {
        if num % i == 0 {
            sum += i;

            if num / i != i {
                sum += num / i;
            }
        }

        if sum > num {
            return true;
        }

        i += 1;
    }

    false
}