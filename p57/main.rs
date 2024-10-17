fn main() {
    println!("{}", square_root_convergence(1_000));
}

fn square_root_convergence(num_terms: i32) -> i32 {
    let mut count = 0;

    let mut frac = (vec![1], vec![1]);
    for i in 0..num_terms {
        // println!("{:?}/{:?}", frac.0, frac.1);
        if frac.0.len() > frac.1.len() {
            count += 1;
        }

        let mut two_denom = add_two_nums(&mut frac.1.clone(), &mut frac.1.clone());
        let mut denom = frac.1;

        let nume_new = add_two_nums(&mut two_denom, &mut frac.0.clone());
        let denom_new = add_two_nums(&mut denom, &mut frac.0);

        frac.0 = nume_new;
        frac.1 = denom_new;
    }

    count
}

fn add_two_nums(num1: &mut Vec<i32>, num2: &mut Vec<i32>) -> Vec<i32> {
    let mut sum_arr = Vec::new();

    let mut carry = 0;
    while !num1.is_empty() || !num2.is_empty() || carry != 0 {
        let i1 = if num1.is_empty() { 0 } else { num1.pop().unwrap() };
        let i2 = if num2.is_empty() { 0 } else { num2.pop().unwrap() };
        let sum = carry + i1 + i2;

        sum_arr.insert(0, sum % 10);

        carry = sum / 10;
    }

    let last_vec = if !num1.is_empty() { num1 } else { num2 };
    while !last_vec.is_empty() {
        let sum = last_vec.pop().unwrap() + carry;

        sum_arr.insert(0, sum % 10);

        if sum >= 10 {
            carry = 1;
        }
    }

    sum_arr
}