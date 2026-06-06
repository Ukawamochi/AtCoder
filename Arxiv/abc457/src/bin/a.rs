use proconio::input;

fn main(){
    input!{
        n: usize,
        a: [i64; n],
        x: usize,
    }
    println!("{}",a[x - 1]);
    
}