use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars
    }
    println!("{}", s.last().unwrap());
}
