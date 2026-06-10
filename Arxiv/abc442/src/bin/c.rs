use proconio::input;

fn main(){
    input!{
        n: usize,
        m: usize,
        ab: [(usize,usize);m],
    }
    let mut rtable = vec![0; n + 1];
    for j in &ab{
        rtable[j.0] += 1;
        rtable[j.1] += 1;
    }
    for i in 1..=n{
        let count = n - 1 - rtable[i];
        let result;
        if count <=2{
            result = 0;
        }else{
            //eprintln!("{count}");
            result = (count) * (count - 1) * (count - 2) / 6;
        }
        if i == n{
            println!("{result}");
        }else{
            print!("{result} ");
        }
        
    }

    
}

