use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u128
    }
    let m = 998244353;
    let mut i = 1;
    let mut res = 0;
    while i <= n {
        let j = (i * 10 - 1).min(n) - i + 1;
        res += (j * (j + 1) / 2) % m;
        res %= m;
        i *= 10;
    }
    println!("{}", res);
}
