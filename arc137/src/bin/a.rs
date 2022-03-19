use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        l: usize,
        r: usize
    }
    let mut res = 0;
    for i in 0..6 {
        for j in 0..6 {
            if r > j {
                let x = l + i;
                let y = r - j;
                if x < y && gcd(x, y) == 1 {
                    res = max(res, y - x);
                }
            }
        }
    }
    println!("{}", res);
}

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}
