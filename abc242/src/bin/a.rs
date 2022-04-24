use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64,
        x: f64
    }
    if x <= a {
        println!("1.000000000000");
    } else if x <= b {
        println!("{}", c / (b - a));
    } else {
        println!("0.000000000000");
    }
}
