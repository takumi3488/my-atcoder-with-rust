use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        x: [usize; q]
    }
    a.sort_unstable();
    for x in x {
        println!("{}", n - a.binary_search(&x).unwrap_or_else(|y| y));
    }
}
