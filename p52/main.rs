fn main() {
    println!("{}", permuted_multiples(6));
}

// try maybe bitwise multiplication
fn permuted_multiples(range: usize) -> usize {
    let mut x = 1;

    // let mut sieve: Vec<bool> = vec![true; 100_000_001];
    let mut magnitude = 1;

    // sieve[0] = false;

    while x <= 0xFFFFFF {
        // if !sieve[x] {
        //     x += 1;
        //     continue;
        // }

        if x == magnitude * 5 {
            magnitude *= 10;
            x = magnitude;
        }
        // println!("{}", x);
        
        let mut digits = Vec::new();

        let mut temp = x;
        while temp > 0 {
            digits.push(temp % 10);
            temp /= 10;
        }

        let size = digits.len();

        if size < (range - 2) as usize {
            x += 1;
            continue;
        }

        digits.sort();

        let mut valid = true;
        for i in 2..=range {
            let i_x = i * x;

            if !is_perm(&digits, i_x) {
                valid = false;
            }
        }

        if valid {
            return x;
        } else {
            // sieve[x] = false;
            // sieve[2 * x] = false;
            // sieve[3 * x] = false;
        }

        x += 1;
    }

    0
}

fn is_perm(digits: &Vec<usize>, i_x: usize) -> bool {
    let mut digits_i_x = Vec::new();

    let mut temp = i_x;
    while temp > 0 {
        digits_i_x.push(temp % 10);
        temp /= 10;
    }

    digits_i_x.sort();

    *digits == digits_i_x
}