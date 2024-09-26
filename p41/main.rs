fn main() {
    println!("{}", pandigital_prime(987654321));
}

fn pandigital_prime(n: usize) -> i32 {
    let mut largest = 0;

    let limit = (n as f64).sqrt() as usize + 1;
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

    let mut results = Vec::new();
    for i in 0..=limit {
        if sieve[i] {
            results.push(i as i32);
        }
    }

    let mut pandigitals: Vec<i32> = Vec::new();

    for i in 1..=9 {
        let mut digits = Vec::new();
        for j in 1..=i {
            digits.push(j);
        }

        let length = digits.len();

        heap_permutation(&mut pandigitals, &mut digits, length, length);
    }

    for i in pandigitals {
        if i <= limit as i32 {
            if results.contains(&i) && i > largest {
                largest = i;
            }
        } else {
            let mut is_prime = true;
            for j in &results {
                if i % *j == 0 {
                    is_prime = false;
                    break;
                }
            }

            if is_prime && i > largest {
                largest = i;
            }
        }
    }

    largest
}

fn heap_permutation(output: &mut Vec<i32>, array: &mut Vec<i32>, size: usize, n: usize) {
	if size == 1 {
        let mut num = 0;
        for i in array {
            num *= 10;
            num += *i;
        }

        output.push(num);

		return;
	}

	for i in 0..size {
		heap_permutation(output, array, size - 1, n);

		if size % 2 == 1 {
			array.swap(0, size - 1);
		} else {
			array.swap(i, size - 1);
		}
	}
}