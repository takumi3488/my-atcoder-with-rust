use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }
    for i in 0..n - 1 {
        a[i + 1] += a[i];
        a[i + 1] %= 360;
    }
    a.sort_unstable();
    let mut res = a[0];
    for i in 0..n - 1 {
        res = max(res, a[i + 1] - a[i]);
    }
    res = max(res, 360 - a[n - 1]);
    println!("{}", res);
}
