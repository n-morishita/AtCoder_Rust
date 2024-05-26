// n*nのビンゴ
// 何回目の数字でビンゴになるかの判定

use proconio::input;

fn main() {
    input! {
        n: i32,
        t: usize,
        a: [i32; t],
    }
    
    let mut row = vec![0; n as usize];
    let mut col = vec![0; n as usize];

    // 斜め
    let mut d1 = 0;
    let mut d2 = 0;

    // ビンゴにならなかった場合の初期値
    let mut ans = -1;
    
    for i in 0..t {
        let r = ( a[i] - 1 ) / n;
        let c = ( a[i] - 1 ) % n;

        row[r as usize] += 1;
        col[c as usize] += 1;

        // 左斜め
        if r == c {
            d1 += 1;
        }

        // 右斜め
        if r + c == n - 1 {
            d2 += 1;
        }

        // どこかの列がビンゴになったかの判定
        if row[r as usize] == n || col[c as usize] == n || d1 == n || d2 == n {
            ans = i as i32 + 1;
            break;
        }
    }
    println!("{}", ans);
}
