use proconio::input;
fn main() {
    input! {
        n: usize,
        c: [usize;n],
    }
    let mut array = vec![0;n];
    for i in 0..n{
        array[c[i] - 1] += 1;
    }
    let mut min_index = 0;
    for i in 0..n{
        if array[i] > array[min_index]{
            min_index = i;
        }
    }
    println!("{}",n - array[min_index]);
}
