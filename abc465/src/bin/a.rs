use proconio::input;
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    if a > b * 2 / 3 {
        println!("Yes");
    }else{
        println!("No");
    }
}
