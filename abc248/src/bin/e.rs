use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [(isize,isize); n]
    }
    if k == 1 {
        println!("Infinity");
        return;
    }
    let mut hm = HashMap::<(isize, isize, isize), usize>::new();
    for i in 0..n - 1 {
        let (xi, yi) = p[i];
        for j in i + 1..n {
            let (xj, yj) = p[j];
            let (a, b) = (yi - yj, -(xi - xj));
            let c = -a * xi - b * yi;
            let g = gcd(gcd(a, b), c);
            *hm.entry((a / g, b / g, c / g)).or_default() += 1;
        }
    }
    let mut res = 0;
    for (_, c) in hm {
        if c >= k * (k - 1) / 2 {
            res += 1;
        }
    }
    println!("{}", res);
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
