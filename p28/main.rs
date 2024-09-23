fn main() {
	println!("{}", number_spiral_diagonal_sum(1001));
}

fn number_spiral_diagonal_sum(dims: i32) -> i32 {
	let mut sum = 1;

	let mut separation = 2;
	let mut iter = 0;

	let mut pos = 1;

	while pos < dims * dims {
		pos += separation;
		iter += 1;

		if iter == 4 {
			separation += 2;
			iter = 0;
		}

		sum += pos;
	}

	sum
}