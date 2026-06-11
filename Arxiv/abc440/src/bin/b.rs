use proconio::input;

fn main(){
    input!{
        n: usize,
    }
    let mut arr = Vec::new();
    for i in 1..=n{
        input!{
            t: usize,
        }
        arr.push((i,t));
        
    }
    arr.sort_by(|a, b| a.1.cmp(&b.1));
    println!("{} {} {}",arr[0].0,arr[1].0,arr[2].0);
}

