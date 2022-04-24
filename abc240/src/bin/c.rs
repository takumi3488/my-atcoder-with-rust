use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize,usize);n]
    }
    let mut dp = vec![vec![false; x + 1]; n+1];
    dp[0][0] = true;
    for i in 0..n {
        let (a, b) = ab[i];
        for j in 0..=x {
            if !dp[i][j] {
                continue;
            }
            if a + j <= x {
                dp[i + 1][a + j] = true;
            }
            if b + j <= x {
                dp[i + 1][b + j] = true;
            }
        }
    }
    println!("{}", if dp[n][x] { "Yes" } else { "No" });
}
