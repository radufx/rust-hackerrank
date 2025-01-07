use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'countLuck' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. STRING_ARRAY matrix
 *  2. INTEGER k
 */

 fn countLuck(matrix: &[String], k: i32) -> String {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut grid: Vec<Vec<char>> = matrix.iter()
        .map(|s| s.chars().collect())
        .collect();
    
    let (mut start_i, mut start_j) = (0, 0);
    let (mut end_i, mut end_j) = (0, 0);
    for i in 0..n {
        for j in 0..m {
            match grid[i][j] {
                'M' => { start_i = i; start_j = j; }
                '*' => { end_i = i; end_j = j; }
                _ => {}
            }
        }
    }
    
    let waves = lee_algorithm(&grid, start_i, start_j, end_i, end_j);
    
    if waves == k {
        "Impressed".to_string()
    } else {
        "Oops!".to_string()
    }
}

fn lee_algorithm(grid: &Vec<Vec<char>>, start_i: usize, start_j: usize, end_i: usize, end_j: usize) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut visited = vec![vec![false; m]; n];
    let mut parent = vec![vec![None; m]; n];
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((start_i, start_j));
    visited[start_i][start_j] = true;
    
    while let Some((i, j)) = queue.pop_front() {
        if i == end_i && j == end_j {
            break;
        }
        
        for (di, dj) in dirs.iter() {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            
            if ni >= 0 && ni < n as i32 && nj >= 0 && nj < m as i32 {
                let ni = ni as usize;
                let nj = nj as usize;
                if !visited[ni][nj] && (grid[ni][nj] == '.' || grid[ni][nj] == '*') {
                    visited[ni][nj] = true;
                    parent[ni][nj] = Some((i, j));
                    queue.push_back((ni, nj));
                }
            }
        }
    }
    
    let mut curr = (end_i, end_j);
    let mut path = vec![];
    while let Some(prev) = parent[curr.0][curr.1] {
        path.push(curr);
        curr = prev;
    }
    path.push((start_i, start_j));
    path.reverse();
    
    let mut waves = 0;
    for (idx, &(i, j)) in path.iter().enumerate() {
        if idx == path.len() - 1 { continue; } 

        let mut available_moves = 0;
        for (di, dj) in dirs.iter() {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            
            if ni >= 0 && ni < n as i32 && nj >= 0 && nj < m as i32 {
                let ni = ni as usize;
                let nj = nj as usize;
                if grid[ni][nj] != 'X' && !((ni, nj) == curr) {
                    if idx > 0 {
                        let prev = path[idx - 1];
                        if (ni, nj) != prev {
                            available_moves += 1;
                        }
                    } else {
                        available_moves += 1;
                    }
                }
            }
        }
        
        if available_moves > 1 {
            waves += 1;
        }
    }
    
    waves
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

        let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let mut matrix: Vec<String> = Vec::with_capacity(n as usize);

        for _ in 0..n {
            let matrix_item = stdin_iterator.next().unwrap().unwrap();
            matrix.push(matrix_item);
        }

        let k = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let result = countLuck(&matrix, k);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
