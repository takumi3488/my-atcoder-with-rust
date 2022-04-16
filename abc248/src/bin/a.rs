use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: usize
    }
    println!("{}", 45 - calc(s))
}

fn calc(n: usize) -> usize {
    if n < 10 {
        return n;
    }
    calc(n / 10) + n % 10
}
