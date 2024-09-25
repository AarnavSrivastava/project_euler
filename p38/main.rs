fn main() {
    println!("{}", pandigital_mutliples());
}

fn pandigital_mutliples() -> i32 {
    let mut largest = 0;

    for i in 1..10_000 {
        let mut product = String::new();

        let mut j = 1;
        while product.len() < 9 {
            product.push_str(&format!("{}", i * j));

            j += 1;
        }

        if product.len() == 9 && !product.contains('0') {
            let mut seen: Vec<usize> = vec![0; 9];
            for i in product.chars() {
                let num_rep = (i as u32 - '0' as u32) as usize;
                if seen[num_rep - 1] == 1 {
                    seen[num_rep - 1] = 1000;
                    break;
                }

                seen[num_rep - 1] = 1;
            }

            let sum: usize = seen.iter().sum();
            if sum != 9 {
                continue;
            }

            let num = product.parse::<i32>().unwrap();

            if largest < num {
                largest = num;
            }
        }
    }

    largest
}