use proconio::{fastout, input, marker::Chars};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars
    }
    let mut a = VecDeque::new();
    a.push_front(n);
    for i in (1..=n).rev() {
        if s[i - 1] == 'L' {
            a.push_back(i - 1);
        } else {
            a.push_front(i - 1);
        }
    }
    for ai in a {
        print!("{} ", ai);
    }
    println!();
}
