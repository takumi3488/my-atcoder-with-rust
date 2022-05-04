use proconio::{input,fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [usize; n]
    }
    let mut res = h[0];
    for i in 0..n-1 {
        if h[i] < h[i+1] {
            res = h[i+1];
        } else {
            break;
        }
    }
    println!("{}", res);
}
