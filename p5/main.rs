fn main() {
    println!("{}", smallest_multiple(20));
}

fn smallest_multiple(range: i32) -> i32 {
    let mut num = 2520;

    while !is_divisible(num, range) {
        num += 2;
    }

    num
}

fn is_divisible(num: i32, range: i32) -> bool {
    for i in 1..(range + 1) {
        if num % i != 0 {
            return false;
        }
    }

    true
}