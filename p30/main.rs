fn main() {
    println!("{}", count_digits_power_sum(5));
}

fn count_digits_power_sum(power: i32) -> i32 {
    let mut sum = 0;
    let limit = calc_limit(power);

    for n in 2..limit {
        if is_digit_power_sum(n, power) {
            sum += n;
        }
    }

    sum
}

fn is_digit_power_sum(n: i32, power: i32) -> bool {
    let mut temp = n;
    let mut sum = 0;

    while temp > 0 {
        sum += (temp % 10).pow(power as u32);
        temp /= 10;
    }

    sum == n
}

fn calc_limit(power: i32) -> i32 {
    let max_digit_val: i32 = 9_i32.pow(power as u32);
    let mut num_digits: i32 = 1;

    while (max_digit_val * num_digits).to_string().chars().count() >= num_digits as usize {
        num_digits += 1;
    }

    max_digit_val * (num_digits - 1)
}