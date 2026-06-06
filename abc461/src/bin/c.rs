use proconio::input;

fn main(){
    input!{
        n: usize,
        k: usize,
        m: usize,
        mut cv: [(usize, usize); n],
    }
    cv.sort_by(|a, b| a.1.partial_cmp(&(b.1)).unwrap());
    let mut select =  vec![0; n + 1];
    let mut result = 0;
    
    let mut index = n - 1;
    for _i in 0..m{
        loop{            
            if select[cv[index].0] == 1{
                if index != 0{
                    index -= 1;
                }
            }else{
                select[cv[index].0] = 1;
                result += cv[index].1;
                cv[index].1 = 0;
                if index != 0{
                    index -= 1;
                }
                break;
            }
        }
    }
    
    let mut count = k - m;
    for i in 0..n{
        if cv[n - 1 - i].1 != 0{
            result += cv[n - 1 - i].1;
            cv[n - 1 - i].1 = 0;
            count -= 1;
        }
        if count == 0{
            break;
        }
    }
    println!("{result}");
    
    
}

