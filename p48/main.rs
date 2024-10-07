fn main() {
    // println!("{:?}", modular_power(2, 32, 100_000));
    println!("{}", final_ten_of_self_powers(1_000, 10_000_000_000));
}

fn final_ten_of_self_powers(limit: i64, modulo: i64) -> i64 {
    let mut n = 0;

    for i in 1..=limit {
        let power = modular_power(i, i, modulo);

        println!("{}^{} % {} = {}", i, i, modulo, power);

        n += power;
        n %= modulo;
    }

    n % modulo
}

fn modular_power(mut x: i64, mut y: i64, p: i64) -> i64 {
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