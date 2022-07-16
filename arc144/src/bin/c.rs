use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: isize,
        k: isize
    }
    if n / 2 < k {
        println!("-1");
        return;
    }
    let mut b: Vec<isize> = vec![];
    for i in 0..=5 {
        b.push(10_i32.pow(i) as isize);
    }
    let mut res = vec![];
    for &b in b.iter().filter(|&&x| x <= n) {
        let s = if n - b + 1 >= k && b - 1 >= k {
            b
        } else if k >= b {
            k + 1
        } else {
            -1
        };
        if s == -1 {
            break;
        }
        let mut new_res = vec![];
        for i in 0..n {
            let m = i as isize + s;
            if m > n {
                new_res.push(m % n);
            } else {
                new_res.push(m);
            }
        }
        if res.len() == 0 {
            res = new_res;
        } else {
            if new_res < res {
                for i in 0..res.len() {
                    if i == new_res.len() || res[i] < new_res[i] {
                        break;
                    }
                    if res[i] > new_res[i] {
                        res = new_res;
                        break;
                    }
                }
            }
        }
    }
    for res in res {
        print!("{} ", res);
    }
    println!();
}
