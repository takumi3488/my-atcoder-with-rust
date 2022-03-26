use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: isize,
        a: [isize; n],
        b: [isize; n],
    }
    let mut dp = vec![(-1_000_000_001, -1_000_000_001); n];
    dp[0] = (a[0], b[0]);
    for i in 1..n {
        let mut flg = true;
        if min((dp[i - 1].0 - a[i]).abs(),(dp[i - 1].1 - a[i]).abs()) <= k {
            flg = false;
            dp[i].0 = a[i];
        }
        if min((dp[i - 1].0 - b[i]).abs(), (dp[i - 1].1 - b[i]).abs()) <= k {
            flg = false;
            dp[i].1 = b[i];
        }
        if flg {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
