use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        r: usize,
        c: usize
    }
    let mut res = 0;
    if r > 1 {
        res += 1;
    }
    if r < h {
        res += 1;
    }
    if c > 1 {
        res += 1;
    }
    if c < w {
        res += 1;
    }
    println!("{}", res);
}
