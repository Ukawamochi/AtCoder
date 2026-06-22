use proconio::input;
fn main() {
    input! {
        n: usize,
        mut tt: [(usize, usize); n],
        q: usize,
        t: [usize;q],
    }
    let mut maxt : Vec<(usize,usize)> = Vec::new();
    tt.sort_by(|a, b| a.1.cmp(&b.1));//昇順
    
    let mut max = 0;
    let mut bt = -1;
    for i in tt{
        if max < i.0{
            max = i.0;
        }
        if i.1 as isize == bt && bt != -1 {
            //eprintln!("bt = tt.1");
            let tmp = maxt.len() - 1;
            maxt[tmp].1 = max;
        }else{
            maxt.push((i.1,max));
        }
        bt = i.1 as isize;
    }
    
    for i in t{
        let target = i;
    
        let mut left = 0;
        let mut right = maxt.len();
    
        while right - left > 1{
            let mid = (left + right) / 2;
            if maxt[mid].0 == target{
                println!("{}",{maxt[mid + 1].1});
                break;
            }
            if maxt[mid].0 < target{
                left = mid;
            }   
            if maxt[mid].0 > target{
                right = mid;
            }
        }
        eprintln!("{}",right);
        
        println!("{}",{maxt[right].1});


        
    }
            
    
}

