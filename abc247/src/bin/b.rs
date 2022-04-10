use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        st: [(String,String);n]
    }
    for sti in &st {
        if st.iter().filter(|x| x.0 == sti.0 || x.1 == sti.0).count() > 1
            && st.iter().filter(|x| x.0 == sti.1 || x.1 == sti.1).count() > 1
        {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
