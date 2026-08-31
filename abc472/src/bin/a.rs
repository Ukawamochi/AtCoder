use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        s: Chars,
    }
    for i in 0..(s.len()){
        if s[i] == 'A'{
            print!("A");
        }else{
            print!(".");
        }
    }
    println!();
}
