use proconio::input;

fn main(){
    input!{
        mut n: i64,
        mut m: i64,
    }
    let mut count = 0;
    loop{
        m = n % m;
        count += 1;
        
        if m == 0{
            break;
        }
    }
    println!("{count}");
    
}

