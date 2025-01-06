use std::io;


fn sum(a: i32, b: i32) -> i32 {
  a + b
}

fn main() {
    let mut a_str = String::new();
    let mut b_str = String::new();

    let mut a: i32 = 0;
    let mut b: i32 = 0;

    io::stdin().read_line(&mut a_str).unwrap();
    io::stdin().read_line(&mut b_str).unwrap();

    a = a_str.trim().parse::<i32>().unwrap();
    b = b_str.trim().parse::<i32>().unwrap();

    println!("{}", sum(a, b));
}

