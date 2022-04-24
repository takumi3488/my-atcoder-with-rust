use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize
    }
    let mut v: Vec<usize> = (2..=b + d).collect();
    for i in 2..=((b + d) as f64).sqrt() as usize {
        let mut j = 2;
        while j * i <= b + d {
            if let Ok(x) = v.binary_search(&(j * i)) {
                v.remove(x);
            }
            j += 1;
        }
    }
    for i in a..=b {
        if (c..=d).all(|j| v.binary_search(&(i + j)).is_err()) {
            println!("Takahashi");
            return;
        }
    }
    println!("Aoki");
}
