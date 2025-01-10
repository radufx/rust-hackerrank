# ATAD source code explanation

This repository contains solutions to various algorithmic problems implemented in Rust.

### 1. Max-Min (medium)

**Problem**: Find the minimum unfairness value by selecting k elements from an array where unfairness is defined as the difference between maximum and minimum elements in the selection.

**Solution Approach**:

- Sort the array first
- Use sliding window of size k
- Calculate difference between max and min in each window

```rust
  sorted_arr[i + k_usize - 1] - sorted_arr[i]
```

- Track the minimum difference found

### 2. Red Knight's Shortest Path (medium)

**Problem**: Find the shortest path for a knight on a chess-like board from start to end position.

**Solution Approach**:

- Use BFS (Breadth-First Search) algorithm or a Lee algorithm with a queue and start from the starting postion

```rust
  queue.push_back(((i_start, j_start), Vec::new()));
```

- Track visited positions using HashSet

```rust
  let mut visited = HashSet::new();
```

- Define possible moves (UL, UR, R, LR, LL, L) and iterate through them to check potential moves

```rust
  for &(row_offset, col_offset, move_name) in &moves {
              let new_row = pos.0 + row_offset;
              let new_col = pos.1 + col_offset;
            ...
  }
```

- Keep track of path during traversal
- If the knight reaches the end position, print the path length and path

```rust
  if pos.0 == i_end && pos.1 == j_end {
      println!("{}", path.len());
      if !path.is_empty() {
          println!("{}", path.join(" "));
      }
      return;
  }
```

- Print "Impossible" if no path exists

### 3. Solve Me First (easy)

**Problem**: Simple addition of two integers.

**Solution Approach**:

- Basic arithmetic addition
- Input handling practice in Rust

### 4. Longest Increasing Subsequence (hard)

**Problem**: Find the length of the longest strictly increasing subsequence in an array.

**Solution Approach**:

- To obtain longest increasing subsequence, we need to maintain a tail of elements that are strictly increasing
- We maintain a tails array where tails[i] represents the smallest value that can end an increasing subsequence of length i+1.
- For each number in the input array: We use binary search to find where this number should be placed in our tails array, and if the number is larger than all endings, we append it
- Length of tails array gives LIS length

### 5. Connected Cells in a Grid (medium)

**Problem**: Find the largest region of connected cells in a matrix where cells are connected horizontally, vertically, or diagonally.

**Solution Approach**:

- DFS implementation
- Iterate through the whole matrix and check if a cell is connected to another cell

```rust
  for i in 0..rows {
      for j in 0..cols {
        ...
      }
  ...
 }
```

- Use a stack to keep track of visited cells and pop the top cell at each iteration

```rust
    let mut stack = vec![(i, j)];

    while let Some((curr_i, curr_j)) = stack.pop()
```

- Check all 8 adjacent cells and mark visited cells to avoid recounting
- Track current size of connected region
- Determine the maximum size of connected region

```rust
    max_size = max_size.max(size);
```

### 6. Count Luck (medium)

**Problem**: Validate if the number of wand waves matches the predicted number for finding path in a maze.

**Solution Approach**:

- Use Lee's algorithm (modified BFS)
- Find starting (M) and ending (\*) positions

```rust
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
```

- Use Lee's algorithm to find the path from start to end and maintain a parent matrix to reconstruct the path
- Reverse the path to find the number of waves by checking the amount of available moves at each step

```rust
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
```

### 7. Divisible Sum Pairs (easy)

**Problem**: Count pairs in array where sum is divisible by k.

**Solution Approach**:

- Use nested loops to check all possible pairs
- Check if sum modulo k equals 0
- Count valid pairs

```rust
    for i in 0..n-1 {
      for j in i+1..n {
        if (ar[i] + ar[j]) % k == 0 {
          number_of_pairs += 1;
        }
      }
    }
```

### 8. Encryption (medium)

**Problem**: Encrypt a string by arranging it in a grid and reading columns.

**Solution Approach**:

- Remove spaces from input

```rust
    let s: String = s.chars().filter(|c| !c.is_whitespace()).collect();
```

- Calculate grid dimensions using square root

```rust
    let rows = (len as f64).sqrt().floor() as usize;
    let cols = (len as f64).sqrt().ceil() as usize;
```

- Arrange characters in grid
- Read and combine columns

```rust
    for col in 0..cols {
        if col > 0 {
            result.push(' ');
        }

        let mut pos = col;
        while pos < len {
            result.push(s.chars().nth(pos).unwrap());
            pos += cols;
        }
    }
```

### 9. Extra Long Factorials (medium)

**Problem**: Calculate large factorials that don't fit in standard integer types.

**Solution Approach**:

- Implement custom multiplication for large numbers
- Store digits in vector
- Handle carry in multiplication

```rust
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
```

- Print the result in reverse order

```rust
    for digit in result.iter().rev() {
        print!("{}", digit);
    }
```
