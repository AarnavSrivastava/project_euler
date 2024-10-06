use std::collections::HashSet;

fn main() {
    println!("{}", distinct_prime_factors(4));
}

fn distinct_prime_factors(num_distinct_factors: usize) -> i32 {
    let mut spf = vec![1; MAX_N];
    sieve(&mut spf);

    let mut n = 1;

    let mut list = Vec::new();
    while list.len() != num_distinct_factors {
        if sieve_factorization(n, &spf).len() == num_distinct_factors {
            list.push(n);
        } else if list.len() > 0 {
            list = Vec::new();
        }

        n += 1;
    }

    list[0]
}

const MAX_N: usize = 10_000_001;

fn sieve(spf: &mut Vec<i32>) {
	spf[0] = 0;
	spf[1] = 1;
    spf[2] = 2;

    for i in (4..MAX_N).step_by(2) {
        spf[i] = 2;
    }

	for i in (3..(MAX_N as i32)).step_by(2) {
		if spf[i as usize] == 1 {
			for j in (i..(MAX_N as i32)).step_by(i as usize) {
				spf[j as usize] = i;
			}
		}
	}
}

fn sieve_factorization(mut x: i32, spf: &Vec<i32>) -> HashSet<i32> {
	let mut ret = HashSet::new();

	while x != 1 {
		ret.insert(spf[x as usize]);

		x /= spf[x as usize];
	}

    ret
}