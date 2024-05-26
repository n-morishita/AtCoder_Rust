// 配列の初期化
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers); // [1, 2, 3, 4, 5]
    
    // 0を10個のvec
    let zeros = vec![0; 10];
    println!("{:?}", zeros); // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    
    // 0-4のvec
    let iter = 0..5;
    let vec_from_iter: Vec<i32> = iter.collect();
    let vec_from_iter: Vec<i32> = (0..5).collect();
    println!("{:?}", vec_from_iter); // [0, 1, 2, 3, 4]

    // 空の配列
    let mut empty_vec: Vec<i32> = Vec::new(); // 空の配列を作成
    empty_vec.push(1);
    empty_vec.push(2);
    println!("{:?}", empty_vec); // [1, 2]

    // 以下、おまけ
    let words = vec!["apple", "banana", "cherry"];
    println!("{:?}", words); // ["apple", "banana", "cherry"]

    let pairs = vec![(1, "one"), (2, "two"), (3, "three")];
    println!("{:?}", pairs); // [(1, "one"), (2, "two"), (3, "three")]

    let array = [1, 2, 3, 4, 5];
    let vec_from_array = Vec::from(array);
    println!("{:?}", vec_from_array); // [1, 2, 3, 4, 5]
}


// パターンマッチ
fn main() {
  let [x, y, z] = [1, 2, 3]; // x=1, y=2, z=3
}


// 昇順 sort(), sort_by()
// sort()
fn main() {
  let mut numbers = [5, 3, 8, 6, 2, 7, 4, 1];
  numbers.sort();
  println!("{:?}", numbers);
  let mut vec = vec![50, 25, 70, 15];
  vec.sort();
  println!("{:?}", vec);
}
// 辞書順も同様に並び替えられる a-z

// sort_by()
fn main() {
    let mut numbers = vec![5, 3, 8, 6, 2, 7, 4, 1];
    numbers.sort_by(|a, b| a.cmp(b));
    println!("{:?}", numbers);
}


// 降順 sort_by()
fn main() {
    let mut numbers = vec![5, 3, 8, 6, 2, 7, 4, 1];
    numbers.sort_by(|a, b| b.cmp(a));
    println!("{:?}", numbers);
}


// 特定の要素の削除 retain
fn main() {
    let mut arr = vec!["pineapple", "apple", "pen"];
    arr.retain(|x| *x != "pineapple");
    println!("{:?}", arr); // ["apple", "pen"]
}
// xのままだと&strになり、違う型同士の比較(&str==str)を行うことはできないため、*xとすることででリファレンスしている(str==str)。


// 合計を求める iter().sum(), iter().map().sum()
// iter().sum()
fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let sum = numbers.iter().sum::<usize>(); // 明示的に型を指定する必要がある
    let sum: i32 = numbers.iter().sum(); // これもok
    println!("{}", sum);
}

// iter().map().sum()
fn main() {
  proconio::input! {
    n: usize,
    p: [(String, usize); n],
  }
  let sum = p.iter().map(|p| p.1).sum::<usize>();
  let sum: i32 = p.iter().map(|p| p.1).sum();
}


// すべての値に対し処理を行う map
fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let squared: Vec<i32> = nums.iter().map(|x| x * x).collect();
    println!("{:?}", squared); // [1, 4, 9, 16, 25]
}


// 特定のものを抽出 filter
fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let even: Vec<i32> = nums.iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", even); // [2, 4]
}


// 短絡論理和（any）と論理積（all）
fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let any_even = nums.iter().any(|x| x % 2 == 0); // いずれかの要素が条件を満たす
    let all_positive = nums.iter().all(|x| *x > 0); // すべての要素が条件を満たす
    println!("Any even: {}", any_even); // Any even: true
    println!("All positive: {}", all_positive); // All positive: true
}


// 部分配列 
fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let slice = &nums[1..4]; // 2, 3, 4
    println!("{:?}", slice);
}


// 重複削除 dedup()
fn main() {
    let mut nums = vec![1, 2, 2, 3, 3, 3, 4];
    nums.dedup();
    println!("{:?}", nums); // [1, 2, 3, 4]
}


// 配列の結合 A.extend(B)
fn main() {
    let mut vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    vec1.extend(vec2);
    println!("{:?}", vec1); // [1, 2, 3, 4, 5, 6]
}


// 配列の反転 reverse()
fn main() {
    let mut nums = vec![1, 2, 3, 4, 5];
    nums.reverse();
    println!("{:?}", nums); // [5, 4, 3, 2, 1]
}


// 反復処理 iter().for_each()
fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    nums.iter().for_each(|x| println!("{}", x));
}


// iter()
fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    nums.iter_mut().enumerate(); // 可変の参照
    nums.iter().enumerate(); // 不変の参照
    nums.into_iter().enumerate(); //元のコレクションを使用できなくなる
}
