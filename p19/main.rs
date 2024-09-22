fn main() {
    println!("{}", count_sundays(1901, 2000));
}

fn count_sundays(start_year: i32, end_year: i32) -> i32 {
    let mut count = 0;

    let mut q = 1;
    let mut m = 13;
    let mut year = start_year;

    while year <= end_year {
        if is_sunday(q, m, year % 100, year / 100) && q == 1 {
            count += 1;
        }

        q += 1;

        if m == 14 && ((year % 4 == 0 && q > 29) || (q > 28)) {
            q = 1;
            m = 3;
        } else if (m == 13 || m == 3 || m == 5 || m == 7 || m == 8 || m == 10 || m == 12) && q > 31 {
            q = 1;
            m += 1;

            if m == 13 {
                println!("New year");
                year += 1;
            }
        } else if (m == 4 || m == 6 || m == 9 || m == 11) && q > 30 {
            q = 1;
            m += 1;
        }
    }

    count
}

fn is_sunday(q: i32, m: i32, k: i32, j: i32) -> bool {
    let congruence = q + 13 * (m + 1) / 5 + k + k / 4 + j / 4 + 5 * j;

    if congruence % 7 == 1 {
        return true;
    }

    false
}