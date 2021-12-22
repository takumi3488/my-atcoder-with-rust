use proconio::input;

fn main() {
    input! {
        n: isize,
        l: isize,
        k: isize,
        mut a: [isize; n]
    }
    a.sort();
    let mut ok = 0;
    let mut ng = l + 1;
    while (ng - ok).abs() > 1 {
        let mid = (ok + ng) / 2;
        if is_ok(&a, l, k, mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}

fn is_ok(a: &Vec<isize>, l: isize, k: isize, mid: isize) -> bool {
    let mut x = 0;
    let mut y = 0;
    for i in a {
        if i - x >= mid && l - i >= mid {
            x = *i;
            y += 1;
        }
    }
    y >= k
}