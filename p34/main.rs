fn main() {
    println!("{}", sum_digit_factorials());
}

fn sum_digit_factorials() -> i32 {
    let mut sum = 0;

    for i in 3..2903040 {
        if is_digit_factorial(i) {
            sum += i;
        }
    }

    sum
}

fn is_digit_factorial(number: i32) -> bool {
    let mut sum = 0;

    let mut num = number;

    while num > 0 {
        sum += calc_factorial(num % 10);
        num /= 10;
    }

    sum == number
}

fn calc_factorial(n: i32) -> i32 {
    if n == 0 {
        return 1;
    }

    let mut factorial = 1;

    for i in 1..=n {
        factorial *= i;
    }

    factorial
}