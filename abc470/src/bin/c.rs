use proconio::input;
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut array = vec![0; n];
    
    for _i in 0..q{
        input! {
            c: usize,
        }
        if c == 1{
            input!{
                d: usize,
            }
            array[d - 1] += 1;
        }else if c == 2{
            for i in 0..n{
                if array[i] >= 1{
                    array[i] -= 1;
                }
            }
        }
        let mut xor = array[0];
        for i in 1..n{
            xor = xor ^ array[i];
        }
        println!("{}",xor);
    }
    
}
