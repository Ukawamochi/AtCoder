use proconio::input;

fn main(){
    input!{
        m: i64,
        d: i64,
    }
    if(m == 1&& d == 7 ){
        println!("Yes");
    }else if(m == 3&& d ==3 ){
        println!("Yes");
    }else if(m ==5 && d ==5 ){
        println!("Yes");
    }else if(m ==7 && d ==7 ){
        println!("Yes");
    }else if(m ==9 && d ==9 ){
        println!("Yes");
    }else{
        println!("No");
    }
    
}
