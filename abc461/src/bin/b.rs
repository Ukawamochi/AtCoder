use proconio::input;

fn main(){
    input!{
        n: usize,
        h: [usize; n],
        g: [usize; n],
    }
    let mut flag = 0;
    for i in 0..n{
        if i != g[h[i] - 1] - 1{
            flag = 1;
        }
    }
    if flag == 1{
        println!("No");
    }else{
        println!("Yes");
    }
    
}

