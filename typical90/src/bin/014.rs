use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        mut b: [i64; n]
    }
    a.sort_unstable();
    b.sort_unstable();
    println!(
        "{}",
        a.iter()
            .zip(b.iter())
            .map(|x| (x.0 - x.1).abs())
            .sum::<i64>()
    );
}
