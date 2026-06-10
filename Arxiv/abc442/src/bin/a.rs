use proconio::input;

fn main(){
    input!{
        s: String,
    }
    let mut result = 0;
    for i in s.chars() {
        if i == 'i' || i == 'j'{
            result += 1;
        }
    }
    println!("{result}");
    
}

