use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        a: [(usize,usize);n]
    }
    let mut dp = vec![0; w + 1];
    for (aw, av) in a {
        for j in (aw..w).rev() {
            dp[j] = dp[j].max(dp[j - aw] + av);
        }
    }
    println!("{}", dp[w]);
}
