use std::collections::HashSet;

fn main() {
    println!("{}", pandigital_products(9));
}

fn pandigital_products(digit_count: i32) -> i32 {
    let mut product_set = HashSet::new();

    let mut digit_array = vec![];
    for i in 1..=digit_count {
        digit_array.push(i);
    }

    let mut perms = Vec::new();
    let n = digit_array.len() as i32;

    heap_permutation(&mut perms, &mut digit_array, n, n);

    for i in perms {
        for len_a in 1..=digit_count {
            for len_b in 1..=(digit_count - len_a) {
                let len_c = digit_count - len_a - len_b;

                if len_c < len_a || len_c < len_b {
                    break;
                }

                let mut pos = 0;
                
                let mut a = 0;
                for j in 0..len_a {
                    a *= 10;
                    a += i[pos];

                    pos += 1;
                }

                let mut b = 0;
                for j in 0..len_b {
                    b *= 10;
                    b += i[pos];

                    pos += 1;
                }

                let mut c = 0;
                for j in 0..len_c {
                    c *= 10;
                    c += i[pos];

                    pos += 1;
                }

                if a * b == c {
                    product_set.insert(c);
                }
            }
        }
    }

    let mut sum = 0;
    for i in product_set {
        sum += i;
    }

    sum
}

fn heap_permutation(results: &mut Vec<Vec<i32>>, array: &mut Vec<i32>, size: i32, n: i32) {
	if size == 1 {
        results.push((*array).clone());
		return;
	}

	for i in 0..size {
		heap_permutation(results, array, size - 1, n);

		if size % 2 == 1 {
			array.swap(0, size as usize - 1);
		} else {
			array.swap(i as usize, size as usize - 1);
		}
	}
}