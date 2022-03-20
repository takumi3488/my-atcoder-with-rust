use proconio::input;
use std::io::{stdout, Write};

fn main() {
    input! {
        n: usize
    };
    let mut a = vec![];
    for i in 1..=(2 * n + 1) {
        a.push(i);
    }
    loop {
        println!("{}", a.pop().unwrap());
        stdout().flush().unwrap();
        input! {
            t: usize
        }
        if t == 0 {
            break;
        }
        a.remove(a.binary_search(&t).unwrap());
    }
}
