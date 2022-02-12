use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        x: [[isize; 3]; n]
    }
    let mut dp = vec![vec![0isize; 3]; n];
    for i in 0..3 {
        dp[0][i] = x[0][i];
    }
    for i in 1..n {
        for j in 0..3 {
            dp[i][j] = max(dp[i - 1][(j + 1) % 3], dp[i - 1][(j + 2) % 3]) + x[i][j];
        }
    }
    println!("{:?}", dp[n - 1].iter().max().unwrap());
}
