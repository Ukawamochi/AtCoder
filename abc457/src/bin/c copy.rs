use proconio::input;

fn main(){
    input!{
        n: usize,
        k: usize,
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
        c: [i64; n],
    }
    
    let mut b = Vec::new();

    
    for i in 0..n{
        for _j in 0..c[i]{
            for k in arrays[i].iter(){
                b.push(k);
            }
        }
    }



    
    println!("{}",b[k - 1]);
    
}