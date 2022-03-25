use num::integer::gcd;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }
    let g = gcd(a, gcd(b, c));
    println!("{}", (a + b + c) / g - 3);
}
