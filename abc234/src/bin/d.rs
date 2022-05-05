use std::collections::BinaryHeap;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: Usize1,
        p: [isize; n]
    }
    let mut bh = BinaryHeap::new();
    let mut res = 0;
    for i in 0..n {
        bh.push(-p[i]);
        if i >= k {
            res = res.max(-bh.pop().unwrap());
            println!("{}", res);
        }
    }
}
