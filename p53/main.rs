fn main() {
    println!("{}", combanitoric_selections(100, 1_000_000));
}

fn combanitoric_selections(range: i32, exceed: i32) -> i32 {
    let mut count = 0;

    for n in 1..=range {
        for r in (0..=n).rev() {
            let combinatoric = calc_combinatoric(n, r, exceed);

            if combinatoric > 1f64 {
                count += 1;
            }
        }
    }

    count
}

fn calc_combinatoric(n: i32, r: i32, exceed: i32) -> f64 {
    let mut numerator = n as f64 / exceed as f64;
    for i in (n - r + 1)..n {
        numerator *= i as f64;
    }

    let mut denominator = 1f64;
    for i in 1..=r {
        denominator *= i as f64;
    }

    numerator / denominator
}