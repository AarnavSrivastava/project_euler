fn main() {
    println!("{}", difference_sum_squares(100))
}

fn difference_sum_squares(limit: i32) -> i32 {
    let mut sum1 = 0;
    let mut sum2 = 0;

    for i in 0..(limit + 1) {
        sum1 += i * i;
        sum2 += i
    }

    sum2 * sum2 - sum1
}