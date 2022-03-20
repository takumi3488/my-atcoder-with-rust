use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars
    }
    let m = 1_000_000_007;
    let t = "atcoder";
    let mut dp = vec![0; 8];
    dp[0] = 1;
    for s in s {
        if let Some(p) = t.find(s) {
            dp[p + 1] = (dp[p] + dp[p + 1]) % m;
        }
    }
    println!("{}", dp[7]);
}
