fn main() {
    println!("{}", digit_cancelling_fractions());
}

fn digit_cancelling_fractions() -> i32 {
    let mut fracs = Vec::new();
    for i in 10..=99 {
        for j in 10..i {
            if i % 10 != 0 && j % 10 != 0 {
                if simplify_fraction(cancel_digits((j, i))) == simplify_fraction((j, i)) {
                    fracs.push((j, i));
                }
            }
        }
    }

    let mut final_frac = (1, 1);
    for i in fracs {
        final_frac = multiply_fractions(final_frac, i);
    }

    final_frac = simplify_fraction(final_frac);

    final_frac.1
}

fn cancel_digits(frac: (i32, i32)) -> (i32, i32) {
    if frac.0 % 10 == frac.1 / 10 {
        return (frac.0 / 10, frac.1 % 10);
    } else if frac.0 / 10 == frac.1 % 10 {
        return (frac.0 % 10, frac.1 / 10);
    } else if frac.0 % 10 == frac.1 % 10 {
        return (frac.0 / 10, frac.1 / 10)
    } else if frac.0 / 10 == frac.1 / 10 {
        return (frac.0 % 10, frac.1 % 10)
    }

    (0, 0)
}

fn simplify_fraction(frac: (i32, i32)) -> (i32, i32) {
    if euclidean_gcd(frac.0, frac.1) == 1 {
        return (frac.0, frac.1);
    }

    let gcd = euclidean_gcd(frac.0, frac.1);

    if gcd == 0 {
        return (1, 1);
    }

    simplify_fraction((frac.0 / gcd, frac.1 / gcd))
}

fn multiply_fractions(frac_1: (i32, i32), frac_2: (i32, i32)) -> (i32, i32) {
    (frac_1.0 * frac_2.0, frac_1.1 * frac_2.1)
}

fn euclidean_gcd(a: i32, b: i32) -> i32 {
	if a == 0 {
		return b;
	}

	euclidean_gcd(b % a, a)
}