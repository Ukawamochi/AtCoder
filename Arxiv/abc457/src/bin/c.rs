use proconio::input;

fn main(){
    input!{
        n: usize,
        k: i64,
    }
    let mut arrays = Vec::new();
    let mut long = Vec::new();
    for _i in 0..n{
        input!{
            l: usize,
            a: [i64; l],
        }
        long.push(l as i64);
        arrays.push(a);
    }
    
    input!{
        c: [i64; n],
    }
    let mut pre_count = 0;
    let mut count = 0;
    let mut result = 0;
    
    for i in 0..n{
        pre_count = count;
        count += long[i] * c[i];
        if pre_count < k && k <= count{
            let mut step = (k - pre_count) % long[i];
            if step == 0{
                step = long[i];
            }
            result = arrays[i][step as usize - 1];
            break;
        }        
    }
    
    



    println!("{}",result);
    
}