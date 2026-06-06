use proconio::input;

fn main(){
    input!{
        s: String,
    }
    let mut result = 0;
    let mut index = 1;
    let len = s.len();
    for i in s.chars(){
        if i == 'C'{
            let mut close = len - index + 1;
            if close > index{
                close = index;
            }

            result += close;
        }
        index += 1;
    }
    println!("{result}");
    
}