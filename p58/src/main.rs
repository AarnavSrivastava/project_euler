fn main() {
    // let mut count = 0;
    // for i in 0..=100_000 {
    //     if is_prime(i, 40) {
    //         count += 1;
    //     }
    // }

    // println!("{}", count);

    println!("{}", number_spiral_primes(0.1));
}

fn number_spiral_primes(ratio: f64) -> i64 {
    let K = 40;

    // let primes = sieve(100_000_000);

    let mut count_primes: f64 = 0.0;
	let mut count_terms: f64 = 1.0;

	let mut separation = 2;
	let mut iter = 0;

	let mut pos = 1;

	loop {
		pos += separation;
		iter += 1;

        if iter != 4 && is_prime(pos, K) {
            count_primes += 1.0;
        }

		count_terms += 1.0;

		if iter == 4 {
            // println!("{} / {} = {}", count_primes, count_terms, count_primes / count_terms);

            if count_primes / count_terms < ratio {
                break;
            }

			separation += 2;
			iter = 0;
		}
	}

	separation + 1
}

use rand::Rng;

// modular exponentiation function
fn power(mut x: i64, mut y: i64, p: i64) -> i64 {
	let mut res = 1;

	x %= p;

	if x == 0 { return 0; }

	while y > 0 {
		if y & 1 == 1 {
            res = modular_multiplication(res, x, p);
		}

		y >>= 1;
		x = modular_multiplication(x, x, p);
	}

	res
}

fn modular_multiplication(mut a: i64, mut b: i64, modulo: i64) -> i64 {
    a %= modulo;
    b %= modulo;

    // if a and b are numbers below 32 bits, then we can just multiply them
    if a <= 0xFFFFFFF && b <= 0xFFFFFF {
        return (a * b) % modulo;
    }

    if b > a {
        (a, b) = (b, a)
    }

    let mut result = 0;
    while a > 0 && b > 0 {
        // if b is odd, then a * b = a * a * (b - 1)
        if b & 1 == 1 {
            result += a;
            result %= modulo;
        }

        a <<= 1;
        a %= modulo;

        b >>= 1;
    }

    result
}

fn miller_rabin_test(mut d: i64, n: i64) -> bool {
	let a = rand::thread_rng().gen_range(2..(n - 1));

	let mut x = power(a, d, n);

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

fn is_prime(n: i64, k: i64) -> bool {
    if n == 2 || n == 3 {
        return true;
    }

	if n < 2 || n % 2 == 0  {
		return false;
	}

	let mut d = n - 1;
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