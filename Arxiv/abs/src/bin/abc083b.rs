
use proconio::input;
fn main() {
    input!{
        n: i32,
        a: i32,
        b: i32,
    }
    let mut count = 0;
    for i in 1..=n{
        let mut target = i;
        let mut sum = 0;
        while target > 0{
            sum += target % 10;
            target /= 10;
        }
        if sum >= a && sum <= b {
            count += i;
        }
    }
    println!("{}",count);
}
