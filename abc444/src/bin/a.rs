use proconio::input;

fn main(){
    input!{
        n: i32,
    }
    let string = n.to_string();
    let char: Vec<char> = string.chars().collect();
    
    if char[0] == char[1] && char[1] == char[2]{
        println!("Yes");
    }else{
        println!("No");
    }
}