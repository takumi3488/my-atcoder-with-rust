use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x:i64
    }
    let mut c = vec![];
    for i in 1..10 {
        c.push(i);
        for d in -9..10 {
            let mut s = i;
            let mut p = i;
            while s < 10i64.pow(17) && 0 <= p + d && p + d <= 9 {
                s *= 10;
                p += d;
                s += p;
                c.push(s);
            }
        }
    }
    c.sort();
    for i in c {
        if i >= x {
            println!("{}", i);
            return;
        }
    }
}
