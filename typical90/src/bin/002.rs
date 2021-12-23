use proconio::{input};

fn main() {
    input! {
        n: usize
    }
    for i in 0..(1 << n) {
        let mut cnt = 0;
        let mut is_ok = true;
        for j in (0..n).rev() {
            if (i>>j) & 1 == 0 {
                cnt += 1;
            } else {
                cnt -=1;
                is_ok &= cnt >= 0;
            }
        }
        is_ok &= cnt == 0;
        if is_ok {
            for j in (0..n).rev() {
                print!("{}", if (i >> j) & 1 == 0 { '(' } else { ')' })
            }
            println!("")
        }
    }
}
