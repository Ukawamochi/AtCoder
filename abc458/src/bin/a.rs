use proconio::input;

fn main(){
    input!{
        s: String,
        n: i64,
    }
    let mut count = 1;
    for i in s.chars() {
        if count <= n || s.len() as i64 - count < n{
            
        }else{
            print!("{i}");
        }
        count += 1;
    }
    
}