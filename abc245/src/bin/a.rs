use proconio::{input,fastout};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize
    }
    if a < c {
        println!("Takahashi")
    } else if a > c {
        println!("Aoki")
    } else if b>d{
        println!("Aoki")
    } else {
        println!("Takahashi")
    }
}
