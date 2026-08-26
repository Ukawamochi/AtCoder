use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut pos : Vec<usize> = Vec::new();
    
    for i in 0..n{
        if s[i] == 'x'{
            pos.push(i);
        }
    }
    let ksum = pos.len();
    for k in 1..=n{
        if k > ksum{
            eprintln!("k = {k} : ans= {n}");
            println!("{}",n);
        }else{
            eprintln!("k = {k} : ans= {}",pos[k-1]);
            println!("{}",pos[k - 1] + 1);
        }
    }
}
