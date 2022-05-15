use proconio::{input,fastout};
use std::{collections::HashSet};

#[fastout]
fn main() {
    input! {
        n: usize,
        st: [(String,isize);n]
    }
    let mut hs = HashSet::new();
    let mut res = (-1,0);
    for i in 0..n {
        if hs.contains(&st[i].0) {
            continue;
        }
        hs.insert(&st[i].0);
        if res.0 < st[i].1 {
            res.0 = st[i].1;
            res.1 = i;
        }
    }
    println!("{}", res.1+1);
}
