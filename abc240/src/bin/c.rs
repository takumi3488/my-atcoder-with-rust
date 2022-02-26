use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [[usize;2]; n]
    }
    for i in 0..(n * n) {
        let mut res = 0;
        for j in 0..n {
            res += ab[j][(i >> j) & 1];
            if res > x {
                break;
            }
        }
        if res == x {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
