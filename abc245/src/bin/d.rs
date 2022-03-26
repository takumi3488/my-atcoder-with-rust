use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize; n+1],
        c: [isize; n+m+1]
    }
    let mut b = vec![0; m + 1];
    b[m] = c[n + m] / a[n];
    for i in 1..=m {
        let lt = (c[n+m-i] - calc(&a, &b, i, m, n)) / a[n];
        b.push(lt);
        print!("{} ", lt);
    }
    println!("{}", b[m]);
}

fn calc(a: &Vec<isize>, b: &Vec<isize>, i: usize, m: usize, n: usize) -> isize {
    let mut res = 0;
    for j in 1..=min(i,n) {
        res += a[n-j] * b[m-i+j];
    }
    res
}
