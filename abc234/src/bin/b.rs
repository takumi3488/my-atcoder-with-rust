use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [(isize,isize);n]
    }
    let mut res = 0f64;
    for q in p.iter().combinations(2) {
        res = res.max((((q[0].0 - q[1].0).pow(2) + (q[0].1 - q[1].1).pow(2)) as f64).sqrt());
    }
    println!("{}", res);
}
