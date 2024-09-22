fn main() {
    println!("{}", quadratic_primes(1000));
}

fn quadratic_primes(limit: i32) -> i32 {
    let mut max = 0;

    let mut best_a = 0;
    let mut best_b = 0;

    for a in -limit..=limit {
        for b in -limit..=limit {
            let mut n = 0;

            while is_prime(n * n + a * n + b) {
                n += 1;
            }

            if n > max {
                best_a = a;
                best_b = b;
                max = n;
            }
        }
    }

    best_a * best_b
}

fn is_prime(num: i32) -> bool {
    if num <= 1 || num % 2 == 0 {
        return false;
    }

    let mut i = 3;
    while i * i <= num {
        if num % i == 0 {
            return false;
        }

        i += 2;
    }

    true
}