use std::collections::HashMap;

fn main() {
    println!("{}", number_letter_counts(1000));
}

fn number_letter_counts(range: usize) -> usize {
    let mut count = 0;
    for i in 1..(range + 1) {
        count += number_to_english(i).len();
    }

    count
}

fn number_to_english(num: usize) -> String {
    let mut number_converted = String::new();
    let ones = vec!["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve",
    "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
    let tens = vec!["twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];

    if num == 1000 {
        return "onethousand".to_string();
    }

    if num < 20 {
        return ones[num].to_string()
    } else if num < 100 {
        number_converted.push_str(tens[num / 10 - 2]);
        number_converted.push_str(ones[num % 10]);
    } else {
        number_converted.push_str(ones[num / 100]);
        number_converted.push_str("hundred");
        
        if num % 100 < 20 {
            if num % 100 != 0 {
                number_converted.push_str("and");
            }

            number_converted.push_str(ones[num % 100]);
        } else {
            number_converted.push_str("and");
            number_converted.push_str(tens[(num % 100) / 10 - 2]);
            number_converted.push_str(ones[num % 10]);
        }
    }

    number_converted
}