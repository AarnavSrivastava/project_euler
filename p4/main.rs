fn main() {
    println!("{}", largest_palindrome_product(3));
}

fn largest_palindrome_product(digits: i32) -> i32 {
    let mut start = 1;

    for i in 0..(digits - 1) {
        start *= 10;
    }

    let mut max = 0;

    for i in (start..(start * 10)).rev() {
        for j in (start..(start * 10)).rev() {
            if is_palindrome(i * j) && i * j > max {
                max = i * j;
            }
        }
    }

    max
}

fn is_palindrome(num: i32) -> bool {
    let s = num.to_string();

    let letters: Vec<char> = s.chars().collect();

    let mut l = 0;
    let mut r = s.len() - 1;

    while l < r {
        if letters[l] != letters[r] {
            return false;
        }

        l += 1;
        r -= 1;
    }

    true
}