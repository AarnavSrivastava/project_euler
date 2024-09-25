fn main() {
    println!("{}", count_combinations(vec![1, 2, 5, 10, 20, 50, 100, 200], 200));
}

fn count_combinations(arr: Vec<i32>, sum: i32) -> usize {
    let results = combination_sum(arr, sum);

    results.len()
}

fn find_numbers(
	arr: &Vec<i32>,
	sum: i32,
	results: &mut Vec<Vec<i32>>, 
	r: &mut Vec<i32>,
	mut i: i32
) {
	if sum == 0 {
		results.push((*r).clone());
		return;
	}

	while i < arr.len() as i32 && sum - arr[i as usize] >= 0 {
		r.push(arr[i as usize]);

		find_numbers(arr, sum - arr[i as usize], results, r, i);
		i += 1;

		r.pop();
	}
}

fn combination_sum(arr: Vec<i32>, sum: i32) -> Vec<Vec<i32>> {
	let mut r = Vec::new();
	let mut results = Vec::new();

	find_numbers(&arr, sum, &mut results, &mut r, 0);

	results
}