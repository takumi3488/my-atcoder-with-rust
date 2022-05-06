use proconio::{input,fastout};

#[fastout]
fn main() {
    input! {
        mut n: usize
    }
    if n >= 42 {
        n += 1;
    }
    println!("{}", n);
}
