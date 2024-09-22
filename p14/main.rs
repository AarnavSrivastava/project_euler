fn main() {
    println!("{}", find_longest_sequence(1_000_000));
}

fn find_longest_sequence(limit: i64) -> i64 {
    let mut max_num = 1;
    let mut max_count = 0;
    for i in 1..limit {
        let mut count = 0;
        let mut num = i;
        while num != 1 {
            num = apply_sequence(num);

            count += 1;
        }

        if max_count < count {
            max_count = count;
            max_num = i;
        }
    }

    max_num
}

fn apply_sequence(num: i64) -> i64 {
    if num % 2 == 0 {
        num / 2
    } else {
        3 * num + 1
    }
}