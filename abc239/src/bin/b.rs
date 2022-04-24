use proconio::{input,fastout};

#[fastout]
fn main() {
    input! {
        mut x: isize
    }
    if x < 0 {
        x -= 9;
    }
    println!("{}", x/10)
}
