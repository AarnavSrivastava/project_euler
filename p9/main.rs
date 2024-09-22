fn main() {
    println!("{}", special_pythagorean_triplet(1000));
}

fn special_pythagorean_triplet(sum: i32) -> i32 {
    // c = sqrt(a^2 + b^2)
    // a + b + c = 1000
    // a + b + sqrt(a^2 + b^2) = 1000
    // c = 1000 - a - b
    // a < b < c
    // b < 1000 - a - b
    // b < 500 - a / 2

    for a in 0..sum {
        if a % 2 == 0 {
            for b in 0..(500 - a / 2) {
                let c = sum - b - a;

                if is_triplet(a, b, c) {
                    return a * b * c;
                }
            }
        }
    }

    0
}

fn is_triplet(a: i32, b: i32, c: i32) -> bool {
    a * a + b * b == c * c
}