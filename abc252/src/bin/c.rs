use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }
    let mut v = vec![vec![0; 10]; 10];
    for s in s {
        for i in 0..10 {
            let j = s[i].to_digit(10).unwrap() as usize;
            v[j][i] += 1;
        }
    }
    let mut res = std::usize::MAX;
    for i in 0..10 {
        let mut t = 0;
        for j in 0..10 {
            if v[i][j] > 0 {
                let mut c = j;
                c += (v[i][j]-1) * 10;
                t = t.max(c);
            }
        }
        if res > t {
            res = t;
        }
    }
    println!("{}", res);
}
