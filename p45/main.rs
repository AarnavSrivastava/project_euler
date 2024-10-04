fn main() {
    println!("{}", triangular_pentagonal_hexagonal());
}

fn triangular_pentagonal_hexagonal() -> i64 {
    let mut curr: i64 = 40755;

    loop {
        curr += 1;

        if is_triangular(curr) && is_pentagonal(curr) && is_hexagonal(curr) {
            break;
        }
    }
    
    curr
}

// n^2 + n - 2T(n) = 0
// a = 1, b = 1, c = -2T(n)
fn is_triangular(n: i64) -> bool {
    let x: f64 = (-1.0 + f64::sqrt((n * 2 * 4  + 1) as f64)) / 2.0;

    let pos = x as i64;

    x - pos as f64 == 0.0
}

// 3n^2 - n - 2P(n) = 0
// a = 3, b = -1, c = -2P(n)
fn is_pentagonal(n: i64) -> bool {
    let x: f64 = (1.0 + f64::sqrt((n * 2 * 3 * 4  + 1) as f64)) / 6.0;

    let pos = x as i64;

    x - pos as f64 == 0.0
}


// 2n^2 - n - H(n) = 0
// a = 2, b = -1, c = -H(n)
fn is_hexagonal(n: i64) -> bool {
    let x: f64 = (1.0 + f64::sqrt((n * 2 * 4 + 1) as f64)) / 4.0;

    let pos = x as i64;

    x - pos as f64 == 0.0
}