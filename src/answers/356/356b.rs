use proconio::input;

fn main() {
    input! {
        n: i32,
        m: i32,
        a: [i32;m],
        v: [[i32;m];n],
    }

    for i in 0..m as usize{
        let total_v: Vec<i32>= v.iter()
                        .map(|inner_vec| inner_vec[i])
                        .collect();
        let total = total_v.iter().sum();
        if a[i] > total {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
