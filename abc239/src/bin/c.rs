use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        p1: (isize,isize),
        p2: (isize,isize)
    }
    let mut v = vec![];
    for a in &[(1, 2), (2, 1)] {
        for b in vec![1, 1, -1, -1].into_iter().combinations(2) {
            v.push((a.0 * b[0], a.1 * b[1]));
        }
    }
    let mut res = vec![];
    for &vi in &v {
        res.push((p1.0 + vi.0, p1.1 + vi.1));
    }
    for &vi in &v {
        if res.iter().any(|&r| r == (p2.0 + vi.0, p2.1 + vi.1)) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
