use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        x: String
    }
    let n = x.len();
    let x = x.as_bytes();
    let mut d = vec![0;n];
    let mut t = 0u32;
    for i in 0..n {
        t += (x[i] - b'0')as u32;
        d[i] = t;
    }
    for i in (1..n).rev() {
        d[i-1] += d[i]/10;
        d[i] %= 10;
    }
    for i in 0..n {
        print!("{}", d[i]);
    }
    println!();
}
