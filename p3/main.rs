fn main() {
    println!("{}", largest_prime_factor(600851475143));
}

fn largest_prime_factor(num: i64) -> i64 {
    let mut max = 1;

    for i in (1..((num as f64).sqrt() as i64)).step_by(2) {
        if num % i == 0 {
            if is_prime(i) {
                max = i;
            }
        }
    }

    max
}

fn is_prime(num: i64) -> bool {
    if num <= 1 {
        return false;
    }

    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }

    true
}