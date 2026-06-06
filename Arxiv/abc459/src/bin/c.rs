use proconio::input;

fn main(){
    input!{
        n: usize,
        q: usize,
    }
    let mut array = [0; 400000];
    let mut num = [0; 400000];
    let mut sum_num = [0; 400000];
    let mut base = 0;
    num[0] = n;
    sum_num[0] = n;
    
    if n == 1{
        for _i in 0..q{
            input!{
                query: (usize, usize),
            }
            if query.0 == 2{
                println!("0");
            }
            
        }
        return
    }

    
    for _i in 0..q{
        input!{
            query: (usize, usize),
        }

        if query.0 == 1{
            let target = query.1 - 1;

            array[target] += 1;
            let k = array[target];
            sum_num[k] += 1;
            num[k] += 1;
            num[k-1] -= 1;

            if num[0 + base] == 0{
                base += 1;
            }
            
        }else{
            let target = query.1;
            let count = sum_num[target + base];
            println!("{count}");
        }
    };
    
}