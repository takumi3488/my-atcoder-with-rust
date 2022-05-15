use proconio::{fastout, input};
use std::usize::MAX;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut dp = vec![vec![MAX;2];n];
    // A1円払わない場合
    dp[0][0] = 0;
    for i in 1..n {
        dp[i][0] = dp[i-1][1];
        dp[i][1] = dp[i-1][0].min(dp[i-1][1])+a[i];
    }
    let mut res = dp[n-1][1];
    // A1円払う場合
    dp = vec![vec![MAX;2];n];
    dp[0][1] = a[0];
    for i in 1..n {
        dp[i][0] = dp[i-1][1];
        dp[i][1] = dp[i-1][0].min(dp[i-1][1])+a[i];
    }
    res = res.min(dp[n-1][0].min(dp[n-1][1]));
    println!("{}", res);
}
