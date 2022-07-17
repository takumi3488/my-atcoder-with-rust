use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize
    }
    let mut r = vec![0; n];
    r[n - 1] = 1;
    let mut b = vec![0; n];
    let mut i = n - 1;
    while i > 0 {
        r[i - 1] += r[i];
        b[i] += r[i] * x;
        r[i - 1] += b[i];
        b[i - 1] += b[i] * y;
        i -= 1;
    }
    println!("{}", b[0]);
}
