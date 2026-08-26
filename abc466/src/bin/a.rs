use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [isize; n],
    }
    let mut flag = 0;
    for i in x{
        if i >= 0{
            flag = 1;
        }
    }
    if flag == 1{
        println!("No");
    }else if flag == 0{
        println!("Yes");
    }
}
