use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n: usize
    }
    let m = 2 * n;
    let mut x = String::from("");
    while n > 0 {
        let d = n.min(4);
        n -= d;
        x = format!("{}{}", d, x);
    }
    println!("{}", m);
    println!("{}", x);
}
