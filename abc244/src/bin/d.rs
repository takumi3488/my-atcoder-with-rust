use proconio::input;

fn main() {
    input! {
        s: (String, String, String),
        t: (String, String, String),
    }
    if (s.0 == t.0 && s.1 == t.1 && s.2 == t.2) || (s.0 == t.2 && s.1 == t.0 && s.2 == t.1) || (s.0 == t.1 && s.1 == t.2 && s.2 == t.0) {
        println!("Yes");
    } else {
        println!("No");
    }
}
