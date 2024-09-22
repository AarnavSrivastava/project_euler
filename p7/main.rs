fn main() {
    println!("{}", find_prime(10001));
}

fn find_prime(find_prime_at: i64) -> i64 {
    let mut count: i64 = 0;

    let mut max_prime = 0;
    let mut num: i64 = 1;

    while count < find_prime_at {
        if is_prime(num) {
            max_prime = num;
            count += 1;
        }

        num += 1;
    }

    max_prime
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