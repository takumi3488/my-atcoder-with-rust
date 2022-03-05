use proconio::input;

fn main() {
    input! {
        n: usize
    }
    let mut dp = vec![vec![0; 9]; n];
    for i in 0..n {
        for j in 0..9 {
            if i == 0 {
                dp[i][j] = 1;
            } else {
                dp[i][j] += dp[i - 1][j] % 998244353;
                if j > 0 {
                    dp[i][j] += dp[i - 1][j - 1] % 998244353;
                }
                if j < 8 {
                    dp[i][j] += dp[i - 1][j + 1] % 998244353;
                }
            }
        }
    }
    let mut res: usize = 0;
    for i in 0..9 {
        res += dp[n - 1][i];
    }
    res = res % 998244353;
    println!("{:?}", &res);
}
