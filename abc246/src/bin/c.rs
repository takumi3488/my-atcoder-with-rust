use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut k: isize,
        x: isize,
        mut a: [isize; n]
    }
    let mut b = vec![];
    a.sort_unstable();
    a.reverse();
    for ai in a {
        if k > 0 {
            let d = min(k, ai / x);
            k -= d;
            b.push(ai % x);
        } else {
            b.push(ai);
        }
    }
    if b.len() <= k as usize {
        println!("0");
    } else {
        b.sort_unstable();
        println!(
            "{}",
            &b[0..(b.len() - k as usize) as usize].iter().sum::<isize>()
        );
    }
}
