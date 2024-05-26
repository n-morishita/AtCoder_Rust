// 絶対値 abs()
fn main() {
    let x = -5;
    let y = x.abs();
    println!("|{}| = {}", x, y); // |-5| = 5
}


// 平方根 sqrt()
fn main() {
    let x = 16.0;
    let y = x.sqrt();
    println!("sqrt({}) = {}", x, y); // sqrt(16) = 4
}
// nの2乗


// 対数 log2()
fn main() {
    let num = 16;
    let result = (num as f64).log2();
    println!("log2({}) = {}", num, result); // log2(16) = 4
}
// 2のn乗


// べき乗(整数) powi()
fn main() {
    let x = 2.0;
    let y = x.powi(3);
    println!("{}^3 = {}", x, y); // 2^3 = 8
}


// べき乗(浮動小数点) powf()
fn main() {
    let x = 2.0;
    let y = x.powf(3.5);
    println!("{}^3.5 = {}", x, y); // 2^3.5 = 11.313708
}


// 丸め関数 (四捨五入) round()
fn main() {
    let x = 2.7;
    let y = x.round();
    println!("round({}) = {}", x, y); // round(2.7) = 3
}


// 丸め関数 (切り捨て) floor()
fn main() {
    let x = 2.7;
    let y = x.floor();
    println!("floor({}) = {}", x, y); // floor(2.7) = 2
}


// 丸め関数 (切り上げ) ceil()
fn main() {
    let x = 2.3;
    let y = x.ceil();
    println!("ceil({}) = {}", x, y); // ceil(2.3) = 3
}


// 最小値 min()
fn main() {
    let a = 3;
    let b = 5;
    let min_val = a.min(b);
    println!("min({}, {}) = {}", a, b, min_val); // min(3, 5) = 3
}


// 配列の最小値
fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    println!("{}", v.iter().min().unwrap());
} 


// 最大値 max()
fn main() {
    let a = 3;
    let b = 5;
    let max_val = a.max(b);
    println!("max({}, {}) = {}", a, b, max_val); // max(3, 5) = 5
}


// 配列の最大値
fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    println!("{}", v.iter().max().unwrap());
}


// 指数関数 exp()
fn main() {
    let x = 1.0;
    let y = x.exp();
    println!("e^{} = {}", x, y); // e^1 = 2.7182818
}


// 自然対数 ln()
fn main() {
    let x = 2.7182818;
    let y = x.ln();
    println!("ln({}) = {}", x, y); // ln(2.7182818) = 1
}