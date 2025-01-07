use std::io::{self, BufRead};
use std::collections::{VecDeque, HashSet};


/*
 * Complete the 'printShortestPath' function below.
 *
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER i_start
 *  3. INTEGER j_start
 *  4. INTEGER i_end
 *  5. INTEGER j_end
 */

 fn printShortestPath(n: i32, i_start: i32, j_start: i32, i_end: i32, j_end: i32) {
    let moves = [
        (-2, -1, "UL"), // Upper Left
        (-2, 1, "UR"),  // Upper Right
        (0, 2, "R"),    // Right
        (2, 1, "LR"),   // Lower Right
        (2, -1, "LL"),  // Lower Left
        (0, -2, "L"),   // Left
    ];
    
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    queue.push_back(((i_start, j_start), Vec::new()));
    visited.insert((i_start, j_start));
    
    while let Some((pos, path)) = queue.pop_front() {
        if pos.0 == i_end && pos.1 == j_end {
            println!("{}", path.len());
            if !path.is_empty() {
                println!("{}", path.join(" "));
            }
            return;
        }
        
        for &(row_offset, col_offset, move_name) in &moves {
            let new_row = pos.0 + row_offset;
            let new_col = pos.1 + col_offset;
            
            if new_row >= 0 && new_row < n && new_col >= 0 && new_col < n 
              && !visited.contains(&(new_row, new_col)) {
                visited.insert((new_row, new_col));
                let mut new_path = path.clone();
                new_path.push(move_name);
                queue.push_back(((new_row, new_col), new_path));
            }
        }
    }
    
    println!("Impossible");
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let i_start = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let j_start = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let i_end = first_multiple_input[2].trim().parse::<i32>().unwrap();

    let j_end = first_multiple_input[3].trim().parse::<i32>().unwrap();

    printShortestPath(n, i_start, j_start, i_end, j_end);
}
