use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut count = 0;
    if n == 1{
        if s[0] == 'x' {
            count += 1;
        }
    }else{
        if s[0] == 'x' && ( s[1] == 'x') {
            count += 1;
        }
        if s[n - 1] == 'x' && ( s[n - 2] == 'x') {
            count += 1;
        }
    }
      
    if n >= 3{
        for i in 1..(n - 1){
            eprintln!("{i}");
            if s[i - 1] == 'x' && s[i + 1] == 'x' &&  s[i] == 'x' {
                count += 1;
            }
        }
    }
    
    println!("{}",count);
    
}