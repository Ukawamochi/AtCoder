use proconio::input;

fn main(){
    input!{
        mut old: usize,
        k: usize,
    }
    let mut year = 0;
    let mut sum = 0;
    loop {
        sum += old;
        if sum >= k{
            println!("{year}");
            break;
        }
        old += 1;
        year+=1;
    }
    
}

