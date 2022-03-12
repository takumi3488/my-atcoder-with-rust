use num::BigInt;
use proconio::input;

fn main() {
    input! {
        _: usize,
        mut x: BigInt,
        s: String
    }
    for si in s.chars() {
        if si.to_string() == String::from("U") {
            x = x / 2;
        } else if si.to_string() == String::from("R") {
            x = x * 2 + 1;
        } else {
            x = x * 2;
        }
    }
    println!("{}", x)
}
