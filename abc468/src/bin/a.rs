use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut count = 0;
    for i in 1..=(n-2){
        if a[i-1] < a[i] && a[i] > a[i+1]{
            count += 1;
        }
    }
    println!("{count}");
}
Ω