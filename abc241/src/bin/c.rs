use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {
        n: usize,
        s_str: [String; n]
    }
    let s: Vec<Vec<char>> = s_str.iter().map(|si| si.chars().collect()).collect();
    let dxdy = [(1, 0), (0, 1), (1, 1), (-1, 1)];
    for x in 0..n {
        for y in 0..n {
            for dxdyi in dxdy {
                if calc(&s, n, x as isize, y as isize, dxdyi.0, dxdyi.1) {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No")
}

fn calc(s: &Vec<Vec<char>>, n: usize, mut x: isize, mut y: isize, dx: isize, dy: isize) -> bool {
    let mut count = 0;
    for _ in 0..6 {
        if !(0 <= min(x, y) && max(x, y) < n as isize) {
            return false;
        }
        if s[x as usize][y as usize].to_string() == "#" {
            count += 1;
        }
        x += dx;
        y += dy;
    }
    count >= 4
}
