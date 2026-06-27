use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        h: usize,
        w: usize,
        c:  [Chars; h],
    }
    let mut h_array = vec![0; h];    
    
    for i in 0..h{
        for j in 0..w{
            if c[i][j] != '.'{
                h_array[i] = 1;
                //eprintln!("#{}",i);
                break;
            }
        } 
    }
    
    let mut start = 0;
    let mut end = h - 1;
    for i in 0..h{
        //eprintln!("h[{}] = {} is 1?",i,h_array[i]);
        if h_array[i] == 0{
            start = i + 1;
            //eprintln!(" {} {} ",start, end);
        }else{
            break;
        }
    }
    for i in (0..h).rev(){
        //eprintln!("h[{}] = {}",i,h_array[i]);
        if h_array[i] == 0{
            end = i - 1;
            //eprintln!(" {} {} ",start, end);
        }else{
            break;
        }
    }
    
    eprintln!("start:{} end:{} ",start, end);

    let mut w_array = vec![0; w];
    for i in 0..w{
        eprintln!("i = {}=======",i);
        for j in start..=end {
            eprintln!("j = {}",j);
            if c[j][i] == '#'{
                eprintln!("[{}][{}] = #",j,i);
                w_array[i] = 1;
                
                break;
            }
        }
    }
    for i in &w_array{
        eprintln!("{i}");
    }
    
    let mut s = 0;
    let mut e = w - 1;
    eprintln!("w start:{} end:{} ",s, e);
    for i in 0..w{
        if w_array[i] == 0{
            s = i + 1;
        }else{
            break;
        }
    }
    eprintln!("s = {}",s);
    for i in (0..w).rev(){
        if w_array[i] == 0{
            e = i - 1;
        }else{
            break;
        }
    }
    eprintln!("e = {}", e);

    
    
    
    
    for i in start..=end{
        for j in s..=e{
            print!("{}",c[i][j]);
        }
        println!();
    }
    
}
