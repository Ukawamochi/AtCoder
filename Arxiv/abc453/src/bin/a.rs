use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        n: usize,
        s: Chars,
    }
    let mut result = String::new();
    let mut start = 0;
    for i in 0..n{
        if s[0] != 'o'{
            break;
        }
        if s[i] == 'o'{
            start += 1;
            continue;
        }else if s[i] != 'o'{
            break;
        }
    }
    for i in (start)..n{
            result.push(s[i]);
    }
    
    if result.len() != 0{
        println!("{}", result);
    }    
}