use std::collections::HashSet;

fn main() {
    println!("{}", integer_right_triangles(1000));
}

fn integer_right_triangles(limit: usize) -> usize {
    let mut max_p = 0;
    let mut max_solns = 0;

    for p in 12..=(limit as i32) {
        let mut solns = HashSet::new();
        for a in 1..(p as i32) {
            for b in 1..(p as i32) {
                let c = p - a - b;

                if c < 0 {
                    continue;
                }

                if a * a + b * b == c * c {
                    solns.insert((a, b, c));
                }
            }
        }

        if solns.len() > max_solns {
            max_solns = solns.len();
            max_p = p as usize;
        }
    }

    max_p
}
