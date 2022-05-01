use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize
    }
    for _ in 0..t {
        sub();
    }
}

fn sub() {
    input! {
        a: u128,
        s: u128
    }
    if a*2 > s {
        println!("No");
        return;
    }
    if a & s - a*2 > 0 {
        println!("No");
    } else {
        println!("Yes");
    }
}
