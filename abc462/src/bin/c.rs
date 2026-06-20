use proconio::input;
fn main() {
    input! {
        n: usize,
        mut xy: [(usize, usize); n],
    }
    xy.sort_by(|a, b| a.0.cmp(&b.0));
    let mut result = 1;
    let mut pre = xy[0].1;
    for i in 1..n{
        if pre > xy[i].1{
            result += 1;
            pre = xy[i].1;
        }
    }
    println!("{result}");
}
