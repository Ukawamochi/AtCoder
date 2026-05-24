use proconio::input;

fn main(){
    input!{
        x: i64,
    }
    let word = "HelloWorld";
    let mut index = 0;
    for i in word.chars(){
        if index !=  x - 1{
            print!("{}",i);
        }
        index += 1;
    }
    println!("");
    
}