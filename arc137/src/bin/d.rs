use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n]
    }
    for _ in 0..m {
        for i in 0..n {
            if i != 0 {
                a[i] = a[i]^a[i-1];
            }
        }
        print!("{} ", a.last().unwrap());
    }
    println!("");
}
