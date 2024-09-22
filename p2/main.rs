fn main() {
    println!("{}", sum_even_fibs(4000000));
}

fn sum_even_fibs(upper_limit: i32) -> i32 {
    let mut sum = 0;

    let mut n1 = 1;
    let mut n2 = 2;

    while n1 < upper_limit && n2 < upper_limit {
        if n2 % 2 == 0 {
            sum += n2;
        }

        let temp = n1;
        n1 = n2;

        n2 += temp;
    }

    sum
}