fn main() {
    println!("{}", number_spiral_primes());
}

fn number_spiral_primes() -> i32 {
    // let primes = sieve(100_000_000);

    let mut count_primes: f64 = 0.0;
	let mut count_terms: f64 = 1.0;

	let mut separation = 2;
	let mut iter = 0;

	let mut pos = 1;

	loop {
		pos += separation;
		iter += 1;

        if primes.contains(&pos) {
            count_primes += 1.0;
        }

		count_terms += 1.0;

		if iter == 4 {
			separation += 2;
			iter = 0;

            println!("{} / {} = {}", count_primes, count_terms - 1.0, count_primes / (count_terms - 1.0));

            if count_primes / (count_terms - 1.0) < 0.1 {
                break;
            }
		}
	}

	separation + 1
}

use rand::Rng;

// modular exponentiation function
fn power(mut x: i32, mut y: i32, p: i32) -> i32 {
	let mut res = 1;

	x %= p;

	if x == 0 { return 0; }

	while y > 0 {
		if y & 1 == 1 {
			res = (res * x) % p
		}

		y >>= 1;
		x = (x * x) % p;
	}

	res
}

fn miller_rabin_test(d: i32, n: i32) -> bool {
	let a = rand::thread_rng().gen_range(2..(n - 1));

	let x = power(a, d, n);

	if x == 1 || x == n - 1 {
		return true;
	}

	while d != n-1 {
		x = (x * x) % n;
		d *= 2;

		if x == 1 {
			return false;
		}

		if x == n - 1 {
			return true;
		}
	}

	false
}

fn is_prime(n: i32, k: i32) -> bool {
	if n < 2 || n == 4 {
		return false;
	} 

	if n < 4 {
		return true;
	}

	let d = n - 1;
	while d % 2 == 0 {
		d /= 2;
		for i in 0..k {
			if !miller_rabin_test(d, n) {
				return false;
			}
		}
	}

	true
}

