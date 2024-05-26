// 二次元配列 : ビンゴ
use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        mut a: [i32; t],
    }
    

    let mut vec: Vec<Vec<i32>>  = Vec::new();
    let num = n as i32;

    let mut min = 1;
    let mut max = num;
    for _i in 1..=n {
        let vec_from_iter: Vec<i32> = (min..=max).collect();
        vec.push(vec_from_iter);
        min += num;
        max += num;
    }

    println!("{:?}", vec)
}

// 一次元配列 (途中まで)
use proconio::input;

fn main() {
    input! {
        n: i32,
        t: usize,
        mut a: [i32; t],
    }
    
    let nn = n * n;
    let mut vec: Vec<i32> = (1..nn).collect();

    // for i in &mut vec {
    for (index, value) in vec.iter_mut().enumerate() {
        if a.contains(value) {
            *value = 0;
        }

        for i in 0..n as usize{
            let mut case1: Vec<i32> = Vec::new();
            case1.push(vec[index + i])
        }
    }

    println!("{:?}", vec)
}

