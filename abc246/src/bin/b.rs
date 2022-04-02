use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: f64,
        y: f64
    }
    let r = (x * x + y * y).sqrt();
    println!("{} {}", x / r, y / r);
}
