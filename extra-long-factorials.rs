use std::io::{self, BufRead};

/*
 * Complete the 'extraLongFactorials' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn extraLongFactorials(n: i32) {
    let mut result = vec![1];
    
    for i in 1..=n {
        multiply(&mut result, i as u32);
    }
    
    for digit in result.iter().rev() {
        print!("{}", digit);
    }
    println!();
}

fn multiply(result: &mut Vec<u32>, num: u32) {
    let mut carry = 0;
    
    for digit in result.iter_mut() {
        let product = *digit as u64 * num as u64 + carry as u64;
        *digit = (product % 10) as u32;
        carry = (product / 10) as u32;
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

