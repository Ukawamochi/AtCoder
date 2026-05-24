use proconio::input;
fn main() {
    input!{
        a: i32,
        b: i32,
        c: i32,
        x: i32,
    }
    
    let mut count = 0;
    
    for i in 0..=a{
        for j in 0..=b{
            for k in 0..=c{
                if 500 * i + 100 * j + 50 * k == x {
                    count += 1;
                }
            }
        }
    }
    println!("{}",count);
    
}
