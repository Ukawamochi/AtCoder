use proconio::input;

fn main(){
    input!{
        n: i64,
    }
    for i in 0..n{
        print!("{}",n - i);
        if n - i != 1{
            print!(",");
        }
    }
    println!("");
}