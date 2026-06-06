use proconio::input;

fn main(){
    input!{
        s: String,
    }
    let first = s.chars().nth(0);
    let end = s.chars().nth(s.len() - 1);
    if first == end {
        println!("Yes");
    }else{
        println!("No");
    }
}