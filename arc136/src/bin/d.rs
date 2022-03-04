use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        mut a: [String; n]
    }
    let mut dp = vec![vec![0;6];10];
    let nums_all = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut nums = HashMap::new();
    for i in 0..=9 {
        hm.insert(i, 0);
        nums.insert(i, &nums_all[0..=i]);
    }
    let mut res = 0;
    for i in (0..n).rev() {
        let mut k = 0;
        for ai in a[i].chars().rev() {
            let b = ai as usize - 48;
            for j in *nums.get(&(9 - b)).unwrap() {
                res += hm.get(j).unwrap();
            }
            hm.insert(b, hm.get(&b).unwrap() + 1);
        }
    }
    println!("{}", res)
}
