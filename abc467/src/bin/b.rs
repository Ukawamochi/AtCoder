use proconio::input;
fn main() {
    input! {
        n: usize,
        abs: [(usize,usize,String); n],
    }
    let mut loss = 0;
    for i in 0..n{
        if abs[i].2 == "keep"{
            loss += abs[i].1 - abs[i].0;
        }
    }
    println!("{}",loss);
}
