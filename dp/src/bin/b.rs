use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [isize; n]
    }
    let mut dp = vec![1isize<<30; n];
    dp[0] = 0;
    for i in 0..n {
        for j in 1..k+1 {
            if i+j<n{
                dp[i+j] = min(dp[i+j], dp[i]+(h[i+j]-h[i]).abs());
            }
        }
    }
    println!("{}",dp[n-1]);
}
