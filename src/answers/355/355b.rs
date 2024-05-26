use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i32; n],
        mut b: [i32; m],
    }

    a.sort();
    let set_a: HashSet<_> = a.iter().cloned().collect();
    
    let mut c: Vec<i32> = Vec::new();
    c.extend(a);
    c.extend(b);
    c.sort();
    c.dedup();

    // cの前から2つずつ取り出す
    for window in c.windows(2) {
        // aにcから取り出したものがあるかの判定
        if window.len() == 2 && set_a.contains(&window[0]) && set_a.contains(&window[1]) {
            println!("Yes");
            return
        }
    }
    println!("No");
}
