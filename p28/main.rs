fn main() {
    let mut spf: Vec<i32> = vec![1; MAX_N];
    sieve(&mut spf);

    // program
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

fn sieve_factorization(mut x: i32, spf: &Vec<i32>) -> Vec<i32> {
	let mut ret = Vec::new();

	while x != 1 {
		ret.insert(0, spf[x as usize]);

		x /= spf[x as usize];
	}

    ret
}