use proconio::input;
fn main() {
    input! {
        n: usize,
    }
    let mut result = 1;
    for _i in 0..n{
        result *= 2;
    }
    println!("{}",result - 2 * n);
}
