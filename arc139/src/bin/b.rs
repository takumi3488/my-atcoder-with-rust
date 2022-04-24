use proconio::{fastout, input};
use std::{cmp::min, usize::MAX};

#[fastout]
fn main() {
    input! {t: usize}
    for _ in 0..t {
        calc();
    }
}

fn calc() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        x: usize,
        y: usize,
        z: usize
    }
    let mut dp = vec![MAX; n + 1];
    dp[0] = 0;
    for i in 0..n {
        dp[i + 1] = min(dp[i + 1], dp[i] + x);
        if i + a <= n {
            dp[i + a] = min(dp[i + a], dp[i] + y);
        }
        if i + b <= n {
            dp[i + b] = min(dp[i + b], dp[i] + z);
        }
    }
    println!("{}", dp[n]);
}
