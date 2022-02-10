use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        h: [isize; n]
    }
    let inf: isize = 1<<30;
    let mut dp = vec![inf; n];
    dp[0] = 0;
    dp[1] = (h[1] - h[0]).abs();
    for i in 2..n {
        dp[i] = min(dp[i-2]+(h[i]-h[i-2]).abs(), dp[i-1]+(h[i]-h[i-1]).abs());
    }
    println!("{}", dp[n-1]);
}
