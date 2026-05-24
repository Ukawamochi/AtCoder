use proconio::input;

fn main(){
    input!{
        n: usize,
        m: i64,
        f: [i64; n],
    }
    let mut q1 = 0;
    let mut q2 = 0;
    let mut isdouble = vec![0; (m + 1) as usize];
    
    
    //その服を着ている人数を求める
    for i in 0..n{
        isdouble[f[i] as usize] += 1;
    }
    
    
    for i in 1..=m{
        if isdouble[i as usize] >= 2 {
            q1 = 1;
        }
    }
    
    for i in 1..=m{
        if isdouble[i as usize] == 0{
            q2 = 1;
        }
    }
    
    
    
    if q1 == 1{
        println!("No");
    }else if q1 == 0{
        println!("Yes");
    }
    if q2 == 1{
        println!("No");
    }else if q2 == 0{
        println!("Yes");
    }
    
    }