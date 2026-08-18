use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut result = vec![0; n];
    let mut sum = vec![0; n];
    
    let mut start = 0;
    let mut count = 0;
    for k in 0..n{
        if s[k] == 'o' && count <= n{
            count += 1;
        }
        sum[k] = count + k;
        if sum[k] >= n {
            sum[k] = n - 1;
        }
    }
    eprintln!("o sum + index array");
    for i in 0..n{
        eprintln!("{}",sum[i]);
    }
    
    for k in 0..n{
        if s[k] == 'x'{
            for i in start..=k{
                eprintln!("result[{i}] = {}",k + 1);
                result[i] = k + 1;
            }
            start = k + 1;
        }else if k == (n - 1){
            for i in start..=k{
                eprintln!("result[{i}] = {}",k + 1);
                result[i] = k + 1;
            }
            start = k;
        }
    }
    
    
    for i in sum{
        eprintln!("answer = result[{}] = {}", i, result[i]);
        println!("{}",result[i]);
        
    }
}
