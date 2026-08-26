use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        cs: [(usize,usize); n],
    }
    let mut max_array= vec![0; m + 1];
    for i in 0..n{
        if max_array[cs[i].0] < cs[i].1 {
            max_array[cs[i].0] = cs[i].1;
        }
    }
    
    for i in 1..m{
        if max_array[i] == 0{
            print!("-1 ");
        }else{
            print!("{} ",max_array[i]);
        }
    }
    if max_array[m] == 0{
        println!("-1");
    }else{
        println!("{}",max_array[m]);
    }
}
