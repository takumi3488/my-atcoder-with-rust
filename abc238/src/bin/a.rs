use proconio::{input,fastout};

#[fastout]
fn main() {
    input! {
        n: usize
    }
    if n >= 2 && n <= 4 {
        println!("No")
    } else {
        println!("Yes")
    }
}
