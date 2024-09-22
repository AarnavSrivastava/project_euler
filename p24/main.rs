fn main() {
    println!("{}", lexicographic_permutations(1_000_000));
}

fn lexicographic_permutations(permutation_to_find: usize) -> String {
    let mut permutations: Vec<String> = Vec::new();
    let digits = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut indices = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let n = indices.len() as i32;

    println!("Genning perms");
    let mut perms = Vec::new();
    heap_permutation(&mut indices, &mut perms, n, n);

    let mut string_perms = Vec::new();

    println!("Converting perms to strings");
    for i in perms {
        let mut string = String::new();
        for j in i {
            let digit = digits[j as usize];
            string.push(digit);
        }

        string_perms.push(string);
    }

    println!("Sorting perms");
    string_perms.sort();

    string_perms[permutation_to_find - 1].clone()
}

fn heap_permutation(array: &mut Vec<i32>, output: &mut Vec<Vec<i32>>, size: i32, n: i32) {
	if size == 1 {
        output.push(array.to_vec());
		return;
	}

	for i in 0..size {
		heap_permutation(array, output, size - 1, n);

		if size % 2 == 1 {
			array.swap(0, size as usize - 1);
		} else {
			array.swap(i as usize, size as usize - 1);
		}
	}
}