use proconio::input;

fn main(){
    input!{
        s: String,
    }
    let leng = s.len();

    let mut check = vec![0; leng - 1];
    for i in 1..leng{
        let pre = s.chars().nth(i - 1 as usize);
        let target = s.chars().nth(i as usize);
        if target == pre{
            check[i - 1] = 1;
        }
    }

    
    let mut tg = 0;
    let mut result = 0;
    let mut seq = 0;
    for i in check.iter(){
        //print!("{} ",i);
    }
    //println!();
    loop{
        if check[tg] == 0{
            seq += 1;
            //println!("seq + 1: {}",seq);
        }
        if check[tg] == 1 || tg == check.len() - 1{
            result += (((seq + 1) * (seq + 1)) - (seq + 1)) / 2;
            //println!("result:{}",result);
            seq = 0;
        }
        
        tg += 1;
        if tg == check.len(){
            break;
        }
    }

    result += leng;    
    let magic = 998244353;
    println!("{}", result % magic);
    
}