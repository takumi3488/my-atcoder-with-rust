use proconio::input;
use proconio::marker::Chars;

fn main(){
  input! {
    n: usize,
    s: [Chars; n],
  }
  let black = '#';
  for i in 0..n {
    for j in 0..n {
      if i+5<n && (0..6).filter(|&k| s[i+k][j]==black).count()>=4
        || j+5<n && (0..6).filter(|&k| s[i][j+k]==black).count()>=4
        || i+5<n && j+5<n && (0..6).filter(|&k| s[i+k][j+k]==black).count()>=4
        || i+5<n && j>=5 && (0..6).filter(|&k| s[i+k][j-k]==black).count()>=4 {
        println!("Yes");
        return;
      }
    }
  }
  println!("No");
}