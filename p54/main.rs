use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::error::Error;
use std::collections::HashMap;

fn main() {
    println!("{}", poker_hands("/Users/aarnavsrivastava/Desktop/_/project_euler/p54/poker.txt"));

    // KC 7H QC 6D 8H 6S 5S AH 7S 8C --> fails

    // let hand_1 = vec![1, 10, 11, 12, 13];
    // let suit_1 = vec!["S", "S", "S", "S", "S"];

    // let hand_2 = vec![8, 6, 8, 8, 6];
    // let suit_2 = vec!['H', "S", "S", "S", "S"];

    // let s1 = eval_hand(hand_1, suit_1);
    // println!();
    // let s2 = eval_hand(hand_2, suit_2);
    // println!("{}", s1);
    // println!("{}", s2);
    // println!("{}", s1 > s2);
}

fn poker_hands(file_path: &str) -> i64 {
    let ranks = HashMap::from([
        ("A", 1),
        ("T", 10),
        ("J", 11),
        ("Q", 12),
        ("K", 13)
    ]);

    let games: Vec<String> = read_file(file_path).unwrap();

    let mut count = 0;

    for game in games {
        let mut p1_rank: Vec<i64> = Vec::new();
        let mut p1_suit: Vec<&str> = Vec::new();

        let mut p2_rank: Vec<i64> = Vec::new();
        let mut p2_suit: Vec<&str> = Vec::new();

        for card in game.split(" ") {
            let rank_str = &card[0..1];

            let rank_int;
            if ranks.contains_key(&rank_str) {
                rank_int = ranks[rank_str];
            } else {
                rank_int = rank_str.parse::<i64>().unwrap();
            }

            let suit = &card[1..2];
            
            if p1_rank.len() < 5 {
                p1_rank.push(rank_int);
                p1_suit.push(suit);
            } else {
                p2_rank.push(rank_int);
                p2_suit.push(suit);
            }
        }

        let p1 = eval_hand(&p1_rank, &p1_suit);
        let p2 = eval_hand(&p2_rank, &p2_suit);


        // if p1 > p2 {
        //     // println!("P1 wins\n");
        // } else {
        //     println!("Hands:\nP1: {:?}; {:?}\nP2: {:?}; {:?}\nP1 score: {}\t\tP2 score: {}", p1_rank, p1_suit, p2_rank, p2_suit, p1, p2);
        //     println!("P2 wins\n");
        // }

        if p1 > p2 {
            count += 1;
        }

        // println!();
    }

    // for i in &p1_wins {
    //     // println!("{}", i);
    // }
    // p1_wins.len() as i64
    count
}

fn eval_hand(hand: &Vec<i64>, suits: &Vec<&str>) -> i64 {
    let multiplier = 1_000_000_000;
    let mut count_face_value = convert_count_face_value(&hand);

    if count_face_value == 1 {
        count_face_value = 11;
    }

    let face_value = convert_face_value(&hand);

    let is_ace_low_straight = face_value == (1 | 2 | 4 | 8 | 16);
    let is_straight = is_ace_low_straight || format!("{:#b}", face_value).contains("11111");
    let is_flush = suits.iter().all(|&suit| suit == suits[0]);

    let tie_breaker = break_tie(&hand);

    // println!("{} = {:#b}", break_tie(&hand), break_tie(&hand));

    // println!("face value count: {}\nface value: {}\nis straight: {}\nis flush: {}\ntie breaker: {}", count_face_value, face_value, is_straight, is_flush, tie_breaker);

    let mut score = multiplier * count_face_value;

    if is_straight || is_flush {
        let mut temp_score = if is_straight { (multiplier as f64 * 9.25) as i64 } else { (multiplier as f64 * 9.75) as i64 };

        if is_straight && is_flush {
            temp_score = multiplier * 15;

            if hand.contains(&1) && hand.contains(&13) {
                temp_score += multiplier * 10;
            }
        }

        if temp_score > score {
            score = temp_score;
        }
    }

    score += tie_breaker;

    score
}

fn break_tie(hand: &Vec<i64>) -> i64 {
    let mut hand: Vec<i64> = (*hand).clone();

    let mut frequencies = HashMap::new();
    for c in 0..hand.len() {
        let mut i = hand[c];
        if i == 1 {
            i = 14;
            hand[c] = 14;
        }

        if frequencies.contains_key(&i) {
            frequencies.insert(i, frequencies[&i] + 1);
        } else {
            frequencies.insert(i, 0);
        }
    }

    let mut sorted_hand = hand.clone();
    sorted_hand.sort_by(|&a, &b| {
        let freq_a = frequencies[&a];
        let freq_b = frequencies[&b];
        
        freq_b.cmp(&freq_a).then_with(|| b.cmp(&a))
    });

    // println!("{:?} sorted to {:?}", hand, sorted_hand);

    let mut result = 0;
    let mut shift = 16;
    
    for i in sorted_hand {
        let res = i << shift;

        // println!("{} << {} = {:#b}", i, shift, res);

        result |= res;
        shift -= 4;
    }

    result
}

fn convert_count_face_value(hand: &Vec<i64>) -> i64 {
    let mut results: Vec<i64> = vec![0; 15];

    let frequencies = hand
          .iter()
          .copied()
          .fold(HashMap::new(), |mut map, val|{
              map.entry(val)
                 .and_modify(|frq|*frq+=1)
                 .or_insert(1);
              map
          });

    for (key, value) in &frequencies {
        for j in 0..*value {
            if *key == 1 {
                results[14] |= 1 << j;
            } else {
                results[*key as usize] |= 1 << j;
            }
        }
    }

    let mut result = format!("{:#b}", results[14])[2..].to_string();
    result = format!("{:0>4}", result);

    for i in (0..=13).rev() {
        let string = &format!("{:#b}", results[i])[2..];
        
        result.push_str(&format!("{:0>4}", string));
    }

    binary_to_i64(result, 15)
}

fn convert_face_value(hand: &Vec<i64>) -> i64 {
    let mut result = 0;

    for i in hand {
        if *i == 1 {
            result |= 1 << 14;
            result |= 1 << 1;
        } else {
            result |= 1 << *i;
        }
    }

    result
}

fn read_file(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut rows = Vec::new();

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        rows.push(line?);
    }

    Ok(rows)
}

fn binary_to_i64(num: String, modulo: i64) -> i64 {
    let mut result = 0;
    let mut pos = 0;
    for i in num.chars().rev() {
        result += (i as u32 - '0' as u32) as i64 * modulo_power(2, pos, modulo);
        result %= modulo;

        pos += 1;
    }

    result
}

fn modulo_power(mut x: i64, mut y: i64, p: i64) -> i64 {
	let mut res = 1;

	x %= p;

	if x == 0 { return 0; }

	while y > 0 {
		if y & 1 == 1 {
			res = (res * x) % p
		}

		y >>= 1;
		x = (x * x) % p;
	}

	res
}