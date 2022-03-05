use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize
    }
    if x <= a {
        println!("1.0")
    } else if x <= b {
        println!("{}",(x as f64)/(b-a))
    } else {
        println!("0.0")
    }
}
