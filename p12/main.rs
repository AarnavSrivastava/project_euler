fn main() {
    println!("{}", triangle_num_divisors(500));
}

fn triangle_num_divisors(num_divisors: usize) -> i64 {
    let mut triangle_num: i64 = 1;
    let mut addition_factor: i64 = 2;

    while find_divisors_length(triangle_num) < num_divisors {
        triangle_num += addition_factor;
        addition_factor += 1;
    }

    triangle_num
}

fn find_divisors_length(num: i64) -> usize {
    let mut divisors = Vec::new();

    if num == 1 {
        return 1;
    }

    let mut i = 1;

    while i * i <= num {
        if num % i == 0 {
            divisors.push(i);
            divisors.push(num / i);
        }

        i += 1;
    }

    divisors.len()
}