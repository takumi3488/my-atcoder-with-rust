use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h]
    }
    let mut sh = vec![0; h];
    let mut vh = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            let aij = a[i][j];
            sh[i] += aij;
            vh[j] += aij;
        }
    }
    for i in 0..h {
        for j in 0..w {
            print!("{} ", sh[i] + vh[j] - a[i][j]);
        }
        println!();
    }
}
