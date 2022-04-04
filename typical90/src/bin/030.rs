use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize
    }
    let mut res = vec![0; n + 1];
    for i in 2..=n {
        if res[i] == 0 {
            let mut j = i;
            while j <= n {
                res[j] += 1;
                j += i;
            }
        }
    }
    println!("{}", res.iter().filter(|&x| x >= &k).count());
}
