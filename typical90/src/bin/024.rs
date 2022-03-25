use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: isize,
        a: [isize; n],
        b: [isize; n]
    }
    let d = a.iter().zip(b.iter()).map(|x| (x.0 - x.1).abs()).sum::<isize>();
    if d <= k && d % 2 == k % 2 {
        println!("Yes")
    } else {
        println!("No")
    }
}
