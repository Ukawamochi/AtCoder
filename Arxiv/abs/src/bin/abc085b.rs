use proconio::input;
fn main() {
    input!{
        n: usize,
        d: [usize; n],
    }
    let mut arr = [0; 101];
    let mut count = 0;
    for i in 0..n{
        if arr[d[i]] == 0{
            arr[d[i]] = 1;
            count+= 1;
        }
    }
    println!("{}",count);
    
}
