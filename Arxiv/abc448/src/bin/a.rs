use proconio::input;

fn main(){
    input!{
        n: usize,
        mut x: i64,
        a: [i64; n],
    }

    for i in 0..n{
        if a[i] < x{
            x = a[i];
            println!("1");
        
        }else{
            println!("0");
        }
    }
    
}