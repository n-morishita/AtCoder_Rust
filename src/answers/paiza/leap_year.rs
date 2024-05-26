// 西暦yが与えられるので、閏年ならYesを、平年ならばNoを出力してください。

// 閏年か平年かは次のような条件によって判定することができます。
// ・西暦が4で割り切れる年は閏年
// ・ただし、100で割り切れる年は平年
// ・ただし、400で割り切れる年は閏年
// ・西暦が4で割り切れない年は平年

// use proconio::input;
use std::io;

fn main() {
    // input! {
    //     y: i32,
    // }
    
    let mut input = String::new(); // 入力を格納するための文字列を初期化
    io::stdin().read_line(&mut input) // 標準入力からの読み取り
        .expect("Failed to read line"); // エラーが発生した場合のメッセージ

    let y: i32 = input.trim().parse()
        .expect("Please type a number!"); // 数値でない場合のエラーハンドリング
    
    if y % 4 == 0 && y % 100 != 0 || y % 400 == 0 {
        println!("Yes");
        return;
    }

    println!("No");
}
