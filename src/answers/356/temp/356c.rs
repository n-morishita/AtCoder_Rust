use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: i32,
        m: i32,
        k: i32,
    }

    for _ in 0..m {
        input! {
            c: usize, 
            mut values: [i32; c], 
            symbol: char, 
        }

    if k == c as i32 && symbol == 'o' {
        println!("{}", 0);
        return;
    }

    let mut v: Vec<Vec<i32>> = Vec::new();
    if symbol == 'o' {
        for i in 0..c {
            v[i].push(1);
        }
    } else {
        for i in 0..c {
            v[i].push(0);
        }
    }

    let vv:Vec<Vec<i32>>  = v.iter().map(|mut x| { x.dedup(); x }).collect();


    }
}
