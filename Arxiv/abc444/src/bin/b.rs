use proconio::input;

fn main(){
    input!{
        nk: (i32, i32),
    }
    
    let mut count = 0;
    for i in 1..=nk.0{
        let num: Vec<char> = i.to_string().chars().collect();
        let mut sum: i32 = 0;
        for i in 0..num.len(){
            sum += num[i].to_digit(10).unwrap() as i32;
        }
        if sum == nk.1{
            count += 1;
        }
    }
    println!("{}",count);
    
}