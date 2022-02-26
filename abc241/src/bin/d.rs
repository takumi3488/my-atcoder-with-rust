use proconio::input;

fn main() {
    input! {
        q: usize
    }
    let mut a = vec![];
    for _ in 0..q {
        input! {n: usize}
        if n == 1 {
            input! {x: usize}
            a.push(x);
            a.sort_unstable();
        } else if n == 2 || n == 3 {
            input! {
                x: usize,
                k: usize
            }
            if k - 1 >= a.len() {
                println!("-1です");
                continue;
            }
            let i = if n == 2 { a.len() - k } else { k - 1 };
            let r = a[i];
            if (n == 2 && r > x) || (n == 3 && r < x) {
                println!("-1でっせ");
                continue;
            }
            println!("{}", r);
        }
    }
}
