// AtCoder
use proconio::input;

fn main() {
    input! {
        n: usize,
        a:[i32;n],
        mut obj: [(String, u32); n]
    }
    
    let ans = n * 2;

    println!("{}", ans);
    println!("{:?}", ans);
}

// Paiza
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    // 数値に変換
    let i: i32 = input.trim().parse()
        .expect("Please type a number!");
        
    println!("{}", i);
}