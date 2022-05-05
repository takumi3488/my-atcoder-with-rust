use proconio::{input,fastout};

#[fastout]
fn main() {
    input! {
        x: isize,
        y: isize
    }
    println!("{}", (y-x+9).max(0)/10);
}
