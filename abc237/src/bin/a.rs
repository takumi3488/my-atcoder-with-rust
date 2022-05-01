use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64
    }
    println!("{}", if n == (n as i32) as i64 { "Yes" } else { "No" });
}
