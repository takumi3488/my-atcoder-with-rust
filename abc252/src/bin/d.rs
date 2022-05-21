use std::collections::HashMap;
use proconio::{input,fastout};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut hm = HashMap::new();
    for a in a {
        *hm.entry(a).or_insert(0) += 1;
    }
    let mut values = vec![];
    for (_,v) in hm {
        values.push(v);
    }
    if values.len() < 3 {
        println!("0");
        return
    }
    let mut res = 0;
    for x in values.iter().combinations(3) {
        res += x[0]*x[1]*x[2];
    }
    println!("{}", res);
}
