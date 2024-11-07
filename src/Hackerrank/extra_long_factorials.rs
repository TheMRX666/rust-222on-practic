use std::io::{self, BufRead};

/*
 * Complete the 'extraLongFactorials' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn extraLongFactorials(n: i32) {
    let mut result = vec![1];

    for i in 2..=n {
        multiply_by(&mut result, i);
    }

    for &digit in result.iter().rev() {
        print!("{}", digit);
    }
    println!();
}

fn multiply_by(result: &mut Vec<u32>, num: i32) {
    let mut carry = 0;

    for digit in result.iter_mut() {
        let product = *digit * num as u32 + carry;
        *digit = product % 10;
        carry = product / 10;
    }

    while carry > 0 {
        result.push(carry % 10);
        carry /= 10;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    extraLongFactorials(n);
}
