use proconio::input;
fn main() {
    input! {
        n: usize,
        l: [isize;n],
    }
    let mut sum = vec![0;n];
    let mut count = 0;
    for i in 0..n{
        count += l[i];
        sum[i] = count;
    }
    let len = sum[n - 1];
    let mut array = vec![0;n - 1];
    for i in 0..=(n - 2){
        array[i] = ((len - sum[i]) - sum[i] ).abs();
    }
    let mut min = array[0];
    for i in 0..(n-1){
        if array[i] < min {
            min = array[i];
        }
    }
    println!("{min}");
}