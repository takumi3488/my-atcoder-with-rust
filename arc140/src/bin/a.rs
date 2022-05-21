use proconio::{input,fastout, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars
    }
    let mut res = std::usize::MAX;
    for i in 1..n {
        let mut l = k;
        let mut j = 0;
        while l > 0 && j <= n {
            s[j..i*j]
        }
    }
}
