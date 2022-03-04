use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n]
    }
    a.sort_unstable();
    b.sort_unstable();
    if a == b {
        println!("Yes")
    } else {
        println!("No")
    }
}
