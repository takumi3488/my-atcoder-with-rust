use num::integer::cbrt;
use proconio::{fastout, input};
use std::{cmp::min, isize::MAX};

#[fastout]
fn main() {
    input! {
        n: usize
    }
    let mut r = (cbrt(n) + 1) as isize;
    let mut res = MAX;
    for i in 0..=r {
        let mut l = -1;
        while r - l > 1 {
            let m = (r + l) / 2;
            let xx = x(i, m);
            if xx < n as isize {
                l = m;
            } else {
                res = min(xx, res);
                r = m;
            }
        }
    }
    println!("{}", res);
}

fn x(a: isize, b: isize) -> isize {
    a * a * a + a * a * b + a * b * b + b * b * b
}
