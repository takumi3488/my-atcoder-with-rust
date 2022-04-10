use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        a: [usize; n]
    }
    let mut res = 0;
    let mut flgs = (false, false);
    for i in 0..n {
        for j in i..n {
            let aj = a[j];
            if x == aj {
                flgs.0 = true;
            }
            if y == aj {
                flgs.1 = true;
            }
            if aj < y || x < aj {
                if flgs.0 && flgs.1 {
                    res += calc(j - i + 1);
                }
                flgs = (false, false);
                break;
            }
        }
        if flgs.0 && flgs.1 {
            res += calc(n - i + 1);
        }
        flgs = (false, false);
    }
    println!("{}", res);
}

fn calc(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    return n + calc(n - 1);
}
