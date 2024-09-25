fn main() {
    println!("{}", double_base_palindromes(1_000_000));
}

fn double_base_palindromes(limit: usize) -> usize {
    let mut sum = 0;

    for i in 0..=limit {
        if is_palindrome(i.to_string()) && is_palindrome(format!("{:b}", i)) {
            sum += i
        }
    }

    sum
}

fn is_palindrome(text: String) -> bool {
    let n = text.len();
    let chars: Vec<char> = text.chars().collect();

    for i in 0..(n / 2) {
        if chars[i] != chars[n - i - 1] {
            return false;
        }
    }

    true
}