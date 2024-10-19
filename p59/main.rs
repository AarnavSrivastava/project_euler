use std::fs::*;
use std::io::{prelude::*, BufReader};
use std::error::Error;

fn main() {
    println!("{}", xor_decryption("/Users/aarnavsrivastava/Desktop/_/project_euler/p59/0059_cipher.txt"));
}

fn xor_decryption(file_path: &str) -> i32 {
    let encrypted = read_cipher(file_path).unwrap();
    let words = read_file("/Users/aarnavsrivastava/Desktop/_/project_euler/p59/words.txt").unwrap();

    // a ^ b = c
    // say c = num and a = ASCII code
    // need to find some key [b1, b2, b3] such that a = c^b(n % 3) forms a coherent word
    // the encryption key consists of three lower case characters
    // key = ['e', 'x', 'p']

    // used to identify possioble keys
    // for b1 in 'a'..='z' {
    //     for b2 in 'a'..='z' {
    //         for b3 in 'a'..='z' {
    //             // println!("{}", b3);
    //             let key = vec![b1, b2, b3];
    //             // println!("{:?}", key);

    //             let mut sentence = String::new();
            
    //             for pos in 0..encrypted.len() {
    //                 let b = key[pos % 3];
    //                 let c = encrypted[pos];

    //                 let a = c ^ b as u8;

    //                 let character = a as char;

    //                 // print!("{}: ", character);

    //                 // valid = a >= '0' as u8 && a <= '9' as u8;
    //                 // print!("{} ", valid);
    //                 // valid |= a >= 'a' as u8 && a <= 'z' as u8;
    //                 // print!("{} ", valid);
    //                 // valid |= a >= 'A' as u8 && a <= 'Z' as u8;
    //                 // print!("{} ", valid);
    //                 // valid |= a == ' ' as u8 || a == '.' as u8 || a == ',' as u8 || a == '?' as u8 || a == '!' as u8;
    //                 // print!("{}", valid)
    //                 // valid |= a == ';' as u8 || a == '-' as u8 || a == '\'' as u8;
    //                 // print!("{} ", valid);
    //                 // valid |= a == '(' as u8 || a == ')' as u8;
    //                 // print!("{} ", valid);
    //                 // println!("\n");

    //                 sentence.push_str(&format!("{}", character));
    //             }

    //             sentence = sentence.to_lowercase();

    //             let sentence = format!("key: {:?};\nsentence: {}\n\n", key, sentence);

    //             let mut count = 0;
    //             for word in &words {
    //                 if sentence.contains(word) {
    //                     count += 1;
    //                 }

    //                 if count == 10 {
    //                     println!("{}", sentence);
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    // }

    let mut sum = 0;

    let key = vec!['e', 'x', 'p'];

    for pos in 0..encrypted.len() {
        let b = key[pos % 3];
        let c = encrypted[pos];

        let a = c ^ b as u8;

        sum += a as i32;
    }

    sum
}

fn read_cipher(file_path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut nums = Vec::new();

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        for num in line?.split(",") {
            nums.push(num.parse().unwrap());
        }
    }

    Ok(nums)
}

fn read_file(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut words = Vec::new();

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let length = line.len();
        if length > 3 {
            words.push(line);
        }
    }

    Ok(words)
}