use proconio::input;
fn main() {
    input!{
        n: usize,
        mut a: [i32; n],
    }
    let mut count = 0;
    loop {
        let mut is_even = 0;
        for i in a.iter(){
            if i % 2 == 0{
                is_even += 1;
            }
        }
        if is_even == n{
            count += 1;
            for j in 0..a.len() {
                a[j] /= 2;
            }
        }else{
            break;
        }
    }
    println!("{}",count);
}
