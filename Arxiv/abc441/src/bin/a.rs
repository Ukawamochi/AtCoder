use proconio::input;

fn main(){
    input!{
        p: isize,
        q: isize,
        x: isize,
        y: isize,
    }
    if p <= x && x <= p + 99 && q <= y && y <= q + 99{
        println!("Yes");
    }else{
        println!("No");
    }
    
}

