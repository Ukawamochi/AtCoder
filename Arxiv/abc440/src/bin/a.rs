use proconio::input;

fn main(){
    input!{
        x: usize,
        y: usize,
    }
    let mut result = x;
    for _i in 0..y{
        result *=2;
    }
    println!("{}",result);
}

