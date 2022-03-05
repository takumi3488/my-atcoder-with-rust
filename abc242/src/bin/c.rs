use proconio::input;

fn main() {
    input! {
        n: usize
    }
    let mut dp = vec![vec![0;9];n];
    for i in 0..n {
        for j in 1..=9 {
            if i == 0 {
                dp[i][j] = 1;
            } else {
                dp[i][j] += dp[i-1][j];
                if j > 1 {
                    dp[i][j] += dp[i-1][j-1];
                }
                if j < 9 {
                    dp[i][j] += dp[i-1][j+1];
                }
            }
        }
    }
    let mut res = 0;
    for i in 1..=9 {
        res += dp.last()[i];
    }
    println!("{}",res);
}
