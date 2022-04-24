use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut k: usize,
        x: usize,
        mut a: [usize; n]
    }
    for i in 0..n {
        let mk = min(k, a[i] / x);
        a[i] -= mk * x;
        k -= mk;
        if k == 0 {
            break;
        }
    }
    a.sort_unstable();
    if k > 0 {
        for i in (if n > k { n - k } else { 0 }..n).rev() {
            a[i] = 0;
        }
    }
    println!("{}", a.iter().sum::<usize>());
}
