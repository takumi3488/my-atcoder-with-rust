use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut t: [u32;n]
    }
    let mut a = vec![0; n];
    a[0] = 2usize.pow(t[0]);
    for i in 1..n {
        let mut j = t[i] as usize;
        a[i] = a[i - 1] & !(2usize.pow(t[i]) - 1);
        if a[i] & 1 << j > 0 {
            j += 1;
        }
        a[i] += 1 << j;
    }
    println!("{}", a[n - 1]);
}
