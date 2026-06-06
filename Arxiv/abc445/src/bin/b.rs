use proconio::input;

fn main(){
    input!{
        //nは奇数
        n: usize,
        s: [String; n],
    }
    let mut max_length = 0;
    for i in 0..s.len(){
        if s[i].len() > max_length {
            max_length = s[i].len();
        }
    }
    for i in 0..s.len(){
        let k = (max_length - s[i].len()) / 2;
        for _j in 0..k{
            print!(".");
        }
        print!("{}",s[i]);
        for _j in 0..k{
            print!(".");
        }
        println!();
    }
    
}