use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }
    a.sort_unstable();
    let mut hs = HashSet::new();
    for ai in a {
        hs.insert(ai);
    }
    for i in 0..=2000 {
        if !hs.contains(&i) {
            println!("{}", i);
            break;
        }
    }
}
