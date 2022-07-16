use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: isize,
        b: isize,
        mut aa: [isize; n]
    }
    aa.sort_unstable();
    let mut l = aa[0].clone();
    let mut r = aa[n - 1].clone();
    while r - l > 1 {
        let m = (r + l) / 2;
        let mut x = 0;
        let mut y = 0;
        for &ai in &aa {
            if ai < m {
                x += (m - ai - 1) / a + 1;
            } else {
                y += (ai - m) / b;
            }
        }
        if x <= y {
            l = m;
        } else {
            r = m;
        }
    }
    println!("{}", l);
}
