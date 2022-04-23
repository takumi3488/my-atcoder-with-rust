use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }
    let mx = 200_001;
    let mut v: Vec<usize> = vec![0; mx + 1];
    for &ai in &a {
        v[ai] += 1;
    }
    let mut res = 0;
    for i in 1..=mx {
        let mut j = 1;
        while i * j < mx {
            res += v[i] * v[j] * v[i * j];
            j += 1;
        }
    }
    println!("{}", res);
}
