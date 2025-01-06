use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'connectedCell' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY matrix as parameter.
 */

 fn connectedCell(matrix: &[Vec<i32>]) -> i32 {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut matrix_copy = matrix.to_vec();
    let mut max_size = 0;
    
    for i in 0..rows {
        for j in 0..cols {
            if matrix_copy[i][j] == 1 {
                let mut size = 0;
                let mut stack = vec![(i, j)];
                
                while let Some((curr_i, curr_j)) = stack.pop() {
                    if matrix_copy[curr_i][curr_j] != 1 {
                        continue;
                    }
                    
                    matrix_copy[curr_i][curr_j] = 2;
                    size += 1;
                    
                    for di in -1..=1 {
                        for dj in -1..=1 {
                            let new_i = curr_i as i32 + di;
                            let new_j = curr_j as i32 + dj;
                            
                            if new_i >= 0 && new_i < rows as i32 && 
                              new_j >= 0 && new_j < cols as i32 {
                                let new_i = new_i as usize;
                                let new_j = new_j as usize;
                                if matrix_copy[new_i][new_j] == 1 {
                                    stack.push((new_i, new_j));
                                }
                            }
                        }
                    }
                }
                
                max_size = max_size.max(size);
            }
        }
    }
    
    max_size
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let m = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut matrix: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        matrix.push(Vec::with_capacity(m as usize));

        matrix[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = connectedCell(&matrix);

    writeln!(&mut fptr, "{}", result).ok();
}
