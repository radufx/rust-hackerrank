use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'longestIncreasingSubsequence' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn longestIncreasingSubsequence(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }
    
    let mut tails: Vec<i32> = Vec::new();
    
    for &num in arr {
        match tails.binary_search(&num) {
            Ok(_) => continue,
            Err(pos) => {
                if pos == tails.len() {
                    tails.push(num);
                } else {
                    tails[pos] = num;
                }
            }
        }
    }
    
    tails.len() as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut arr: Vec<i32> = Vec::with_capacity(n as usize);

    for _ in 0..n {
        let arr_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        arr.push(arr_item);
    }

    let result = longestIncreasingSubsequence(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
