// GPTで解説のC++から変換

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 最初の行から n, m, k を読み取る
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
    let n = nums.next().unwrap();
    let m = nums.next().unwrap();
    let k = nums.next().unwrap();

    let mut key = vec![vec![0; n]; m];
    let mut r = vec![String::new(); m];

    // m 行の入力を読み取る
    for i in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let c = parts.next().unwrap().parse::<usize>().unwrap();

        for _ in 0..c {
            let a = parts.next().unwrap().parse::<usize>().unwrap();
            key[i][a - 1] = 1;
        }
        r[i] = parts.next().unwrap().to_string();
    }

    let mut res = 0;
    // 全てのビットマスクを試す
    for i in 0..(1 << n) {
        let mut tf = vec![0; n];
        for j in 0..n {
            if i & (1 << j) != 0 {
                tf[j] = 1;
            }
        }

        let mut jud = true;
        for j in 0..m {
            let mut ck = 0;
            for p in 0..n {
                if key[j][p] == 1 && tf[p] == 1 {
                    ck += 1;
                }
            }
            if (ck >= k && r[j] == "x") || (ck < k && r[j] == "o") {
                jud = false;
                break;
            }
        }
        if jud {
            res += 1;
        }
    }

    println!("{}", res);
}

// 
// ---
// 

// proconioを使用した場合
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let mut key = vec![vec![0; n]; m];
    let mut r = vec![String::new(); m];

    // m行の入力を読み取る
    for i in 0..m {
        input! {
            c: usize,
            a: [usize; c],
            ri: String,
        }
        for &a_val in &a {
            key[i][a_val - 1] = 1;
        }
        r[i] = ri;
    }

    let mut res = 0;
    // 全てのビットマスクを試す
    for i in 0..(1 << n) {
        let mut tf = vec![0; n];
        for j in 0..n {
            if i & (1 << j) != 0 {
                tf[j] = 1;
            }
        }

        let mut jud = true;
        for j in 0..m {
            let mut ck = 0;
            for p in 0..n {
                if key[j][p] == 1 && tf[p] == 1 {
                    ck += 1;
                }
            }
            if (ck >= k && r[j] == "x") || (ck < k && r[j] == "o") {
                jud = false;
                break;
            }
        }
        if jud {
            res += 1;
        }
    }

    println!("{}", res);
}
