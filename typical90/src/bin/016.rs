use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; 3]
    }
    a.sort();
    let mut mi = std::usize::MAX;
    for i in 0..=(n / a[2]) {
        for j in 0..=((n - a[2] * i) / a[1]) {
            if (n - a[2] * i - a[1] * j) % a[0] == 0 {
                mi = std::cmp::min(i + j + (n - a[2] * i - a[1] * j) / a[0], mi);
            }
        }
    }
    println!("{}", mi);
}
