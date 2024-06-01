use proconio::input;

fn main() {
    input! {
        n: i32,
        l: usize,
        r: usize,
    }

    let mut num: Vec<i32> = (1..=n).collect();
    let start = l - 1;
    num[start..r].sort_by(|a, b| b.cmp(a));
    
    for i in num {
        print!("{} ",i);
    }
}
