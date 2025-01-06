use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'maxMin' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER k
 *  2. INTEGER_ARRAY arr
 */

fn maxMin(k: i32, arr: &[i32]) -> i32 {
    let mut sorted_arr = arr.to_vec();
    let k_usize = k as usize;
    sorted_arr.sort();
    
    let mut min_unfairness = i32::MAX;
    
    for i in 0..=(sorted_arr.len() - k_usize) {
        let unfairness = sorted_arr[i + k_usize - 1] - sorted_arr[i];
        min_unfairness = min_unfairness.min(unfairness);
    }
    
    min_unfairness
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let k = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut arr: Vec<i32> = Vec::with_capacity(n as usize);

    for _ in 0..n {
        let arr_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        arr.push(arr_item);
    }

    let result = maxMin(k, &arr);

    writeln!(&mut fptr, "{}", result).ok();
}
