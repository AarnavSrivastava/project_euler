fn main() {
    println!("Sum: {}", sum_5_or_3(1000));
}

fn sum_5_or_3(range: i32) -> i32 {
    let mut sum = 0;
    for i in 0..range {
        if i % 5 == 0 || i % 3 == 0 {
            sum += i;
        }
    }

    sum
}