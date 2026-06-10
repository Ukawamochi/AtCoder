use proconio::input;

fn main(){
    input!{
        q: usize,
        a: [usize; q],
    }
    let mut play = 0;
    let mut vol = 0;
    for i in a{
        if i == 1 {
            vol += 1;
        }else if i == 2{
            if vol >= 1{
                vol -= 1;
            }
        }else if i == 3{
            if play == 0{
                play = 1;
            }else{
                play = 0;
            }
        }
        if play == 1 && vol >= 3{
            println!("Yes");
        }else{
            println!("No");
        }
    }
}

