use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize
    }
    let _mod = 998244353;
    let mut dp = vec![vec![0; k + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=k {
            if i == 1 {
                if j <= m {
                    dp[i][j] = 1;
                }
            } else {
                for l in 1..=m {
                    if l > j - 1 {
                        break;
                    }
                    dp[i][j] = (dp[i][j] + dp[i - 1][j - l]) % _mod;
                }
            }
        }
    }
    println!("{}", dp.last().unwrap().iter().sum::<usize>() % _mod)
}
