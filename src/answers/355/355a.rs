use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    
    let values: Vec<i32> = (0..4).collect(); // [0, 1, 2, 3]
    // indexがaでもbでもないもののvecを作成
    let result: Vec<i32> = values
        .iter()
        .enumerate()
        .filter_map(|(index, &value)| {
            if index != a && index != b {
                Some(value)
            } else {
                None
            }
        })
        .collect();

    if result.len() == 2 {
        let ans = result[1];
        println!("{}", ans);
    } else {
        println!("-1", );
    }
}
