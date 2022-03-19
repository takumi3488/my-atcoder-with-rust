use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut prev = *a.clone().first().unwrap();
    let mut res = vec![0, 0];
    res[prev] = 1;
    let mut cur = (prev, 1);
    for i in 1..n {
        if prev == a[i] {
            cur.1 = cur.1 + 1;
        } else {
            res[prev] = max(res[prev], cur.1);
            cur.0 = a[i];
            cur.1 = 1;
            prev = a[i];
        }
    }
    res[prev] = max(res[prev], cur.1);
    println!("{}", res[0] + res[1] + 1);
}
