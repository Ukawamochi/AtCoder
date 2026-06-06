use proconio::input;

fn main(){
    input!{
        h: i64,
        w: i64,
    }
    if w == 2 &&  h == 1{
        println!("1 1");
        return
    }
    if w == 1 && h == 2{
        println!("1");
        println!("1");
        return
    }
    if w * h == 1{
        println!("0");
        return
    }
    if w == 1{
        for i in 1..=h{
            if i == 1 || i == h{
                println!("1");
            }else{
                println!("2");
            }
        }
        return
    }
    if h == 1{
        for i in 1..=w{
            if i == 1{
                print!("1 ");
            }else if i == w {
                println!("1");
            }else{
                print!("2 ");
            }
        }
        return
    }
    for i in 1..=h{
        for j in 1..=w{
            let mut show = 0;
            if (i == 1 && j == 1) || (i == 1 && j == w) || (i == h &&  j == 1) || (i == h && j == w){
                show = 2;
            }else if i == 1 || i == h || j == 1 || j == w{
                show = 3;
            }else{
                show = 4;
            }
            if j == w {
                println!("{show}");
            }else{
                print!("{show} ");
            }
        }
    }
    
    
}