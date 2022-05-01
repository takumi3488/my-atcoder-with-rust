use std::collections::VecDeque;

use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        mut s: Chars
    }
    let mut t = VecDeque::from(s);
    while t.back() == Some(&'a') {
        if t.front() == Some(&'a') {
            t.pop_front();
        }
        t.pop_back();
    }
    for i in 0..t.len() / 2 {
        if t[i] != t[t.len() - i - 1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
