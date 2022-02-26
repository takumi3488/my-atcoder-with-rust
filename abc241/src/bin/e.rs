use proconio::input;

fn main() {
    input!{
        n: usize,
        k: usize,
        a: [usize; n]
    }
    let mut x = 0;
    for _ in 0..k {
        x += a[x%n];
    }
    println!("{}",x)
}
