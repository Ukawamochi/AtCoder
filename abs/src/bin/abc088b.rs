use proconio::input;
fn main() {
    input!{
        n: usize,
        mut a: [i32; n],
    }
    let mut rest = n;
    let mut alice = 0;
    let mut bob = 0;
    let mut turn = 0;
    while rest > 0{
        let mut max = a[0];
        let mut j = 0;
        for i in 1..n{
            if a[i] > max{
                max = a[i];
                j = i;
            }
            
        }
        a[j] = 0;
        if turn == 0{
            alice += max;
            turn = 1;
        }else if turn == 1{
            bob += max;
            turn = 0;
        }
        
        rest -= 1;
    }
    println!("{}",alice - bob);
}
