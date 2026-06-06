use proconio::input;

fn main(){
    input!{
        n: usize,
        m: usize,
        mut a: [i64; n],
        mut b: [i64; m],
    }
    a.sort();
    b.sort();
    let mut count = 0;
    let mut index = 0;
    for i in a{
        if i * 2 >= b[index]{
            count += 1;
            index +=1;
        }
        
        if index == b.len(){
            break;
        }
    }
    println!("{count}");
}

