use proconio::input;

fn main() {
    input! {
        h: usize, w: usize,
        c: [String; h],
    }
    let c: Vec<Vec<char>> = c.iter().map(|c| c.chars().collect()).collect();
    let mut dp = vec![vec![0; w+1]; h+1];
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            if c[i][j] == '.' {
                dp[i][j] = dp[i+1][j].max(dp[i][j+1]) + 1;
            }
        }
    }
    println!("{}", dp[0][0]);
}
