use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut v = vec![0; 200_002];
    for &a in &a {
        v[a + 1] += 1;
    }
    for i in 1..200_002 {
        v[i] += v[i - 1];
    }
    let mut res = 0;
    for i in 0..n {
        res += v[a[i]] * (n - v[a[i] + 1]);
    }
    println!("{}", res);
}
