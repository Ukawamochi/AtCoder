use proconio::input;
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        a: [[usize;w];h],
        b: [usize; n],
    }
    let mut max = 0;
    for i in 0..h{
        let mut count = 0;
        
        for j in 0..w{
            for k in 0..n{
                if b[k] == a[i][j] {
                    count += 1;
                    break;
                }
            }
        }
        
        if count > max {
            max = count;
        }
    }
    println!("{max}");
}
