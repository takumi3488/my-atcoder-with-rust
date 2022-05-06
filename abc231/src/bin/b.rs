use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n]
    }
    let mut hm = HashMap::new();
    for s in s {
        *hm.entry(s).or_insert(0) += 1;
    }
    println!("{}", hm.iter().max_by_key(|(_, &v)| v).unwrap().0)
}
