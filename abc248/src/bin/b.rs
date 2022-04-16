use num::Float;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
        k: f64
    }
    println!("{}", ((Float::log2(b)-Float::log2(a))/Float::log2(k)).ceil() as usize)
}
