use proconio::{input};

fn main() {
    input! {
        n: isize
    }
    for tmp in 0..(1 << n) { // 上位ビットが文字列の左側に相当
        let mut cnt = 0;
        let mut ok = true;
        for i in (0..n).rev() { // 上位ビットから順に走査
            if (tmp >> i) & 1 == 0 {
                cnt += 1;
            } else {
                cnt -= 1;
                ok &= cnt >= 0;
            }
        }
        ok &= cnt == 0;
        if ok {
            for i in (0..n).rev() { // 上位ビットから順に走査
                print!("{}", if (tmp >> i) & 1 == 0 { '(' } else { ')' })
            }
            println!();
        }
    }
}
