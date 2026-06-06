use proconio::input;

fn main(){
    input!{
        n: i64,
        m: i64,
    }
    let mut num = 0;
    
    if n % 2 == 0{
        num = n / 2;
    }else{
        num = (n / 2) + 1;
    }
    if num < m{
        println!("No");
    }else{
        println!("Yes");
    }
}