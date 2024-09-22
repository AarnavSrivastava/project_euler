use std::env;
use std::fs;

fn main() {
    let mut names = read_names("/Users/aarnavsrivastava/Desktop/_/rust_problems/p22/names.txt");
    println!("{}", names_scores(names));
}

fn names_scores(mut names: Vec<String>) -> i64 {
    names.sort();
    let mut sum: i64 = 0;

    for i in 0..names.len() {
        sum += compute_score(&mut names[i], i + 1);
    }

    sum
}

fn compute_score(name: &mut String, position: usize) -> i64 {
    let mut score: i64 = 0;

    for i in name.to_lowercase().chars() {
        if i == '"' {
            continue;
        }

        score += convert_letter_to_i64(i);
    }

    score *= position as i64;

    score
}

fn read_names(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    contents.split(',').map(|s| s.to_string()).collect()
}

fn convert_letter_to_i64(letter: char) -> i64 {
    let ASCII_LOWER: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 
        'f', 'g', 'h', 'i', 'j', 
        'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't', 
        'u', 'v', 'w', 'x', 'y', 
        'z',
    ];

    for i in 0..ASCII_LOWER.len() {
        if ASCII_LOWER[i] == letter {
            return i as i64 + 1
        }
    }

    0
}