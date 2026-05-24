use proconio::input;

fn main(){
    input!{
        t: usize,
        x: i64,
        a: [i64; t + 1],
    }
    let mut save = a[0];
    println!("{} {}", 0, save);
    for i in 1..=t{
        //println!("{} {}", i, a[i]);
        if (a[i] - save).abs() >= x{
            println!("{} {}", i, a[i]);
            save = a[i];
            //println!("save: {}",save);
        }
    }
    
}