use proconio::input;

fn main(){
    input!{
        n: usize,
    }
    let mut arrays = Vec::new();
    for i in 0..n{
        input!{
            l: usize,
            a: [i64; l],
        }
        arrays.push(a);
    }
    
    input!{
        x: usize,
        y: usize,
    }
    println!("{}",arrays[x - 1][y - 1]);
    
}