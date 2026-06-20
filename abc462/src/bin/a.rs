use proconio::input;
fn main() {
    input! {
        s: String,
    }
    let mut index = 0;
    for i in s.chars(){
        let mut flag = 0;
        for number in 0..=9{
            let n: u32 = i as u32 - 48;
            if n == number {
                flag = 1;
            }
        }
        if flag == 1{
            print!("{}",i);
        }
        if index == s.len(){
            println!();
        }
        index += 1;
    }
}
