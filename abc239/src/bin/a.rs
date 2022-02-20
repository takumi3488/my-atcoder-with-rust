use proconio::input;

fn main() {
    input! {
        h: isize
    }
    println!("{}", ((h*(12800000+h)) as f64).sqrt())
}
