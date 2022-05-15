use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        a: [usize; n]
    }
    let mut res = vec![false; w + 1];
    for &a in &a {
        if a <= w {
            res[a] = true;
        }
    }
    for a in (&a).iter().combinations(2) {
        if a[0] + a[1] <= w {
            res[a[0] + a[1]] = true;
        }
    }
    for a in (&a).iter().combinations(3) {
        if a[0] + a[1] + a[2] <= w {
            res[a[0] + a[1] + a[2]] = true;
        }
    }
    println!("{}", res.iter().filter(|&x| *x).count());
}
