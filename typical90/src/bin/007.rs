use proconio::input;
use std::convert::identity;

fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
        q: usize,
        b: [isize; q]
    }
    a.sort_unstable();
    for &b in &b {
        let i = a.binary_search(&b).unwrap_or_else(identity);
        let ans = if i == 0 {
            a[i] - b
        } else if i == a.len() {
            b - a[i - 1]
        } else {
            (a[i] - b).min(b - a[i - 1])
        };
        println!("{}", ans);
    }
}
