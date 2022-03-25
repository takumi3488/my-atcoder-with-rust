use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String;n]
    }
    let mut hs = HashSet::new();
    for i in 0..n {
        if !hs.contains(&s[i]) {
            hs.insert(&s[i]);
            println!("{}", i + 1);
        }
    }
}
