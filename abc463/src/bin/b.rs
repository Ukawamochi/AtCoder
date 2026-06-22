use proconio::input;
fn main() {
    input! {
        n: usize,
        x: char,
        s: [String;n],
    }
    let mut target = 0;
    if x == 'A' {
        target = 0;
    }else if x == 'B' {
        target = 1;
    }else if x == 'C' {
        target = 2;
    }else if x == 'D' {
        target = 3;
    }else if x == 'E' {
        target = 4;
    }
    let mut flag = 0;
    for i in s{
        let mut index = 0;
        for j in i.chars(){
            if index == target && j == 'o'{
                flag = 1;
            }
            index += 1;
        }
    }
    
    if flag == 0{
        println!("No");
    }else{
        println!("Yes");
    }
}
