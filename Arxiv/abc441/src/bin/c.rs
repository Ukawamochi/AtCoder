use proconio::input;

fn main(){
    input!{
        n: usize,
        k: usize,
        x: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut al = 0;
    let mut cup = n - k;
    let mut flag = 0;
    for i in (0..k).rev(){
        al += a[i];
        cup += 1;
        if al >= x{
            flag = 1;
            break;
        }
        
    }
    if flag == 1{
        println!("{cup}");
    }else{
        println!("-1");
    }
}

