use proconio::input;

fn main(){
    input!{
        mut h: i64,
        mut w: i64,
        q: usize,
        query: [(i64, i64); q],
    }
    for i in 0..q{
        let mut result = 0;
        if query[i].0 == 1{
            result = query[i].1 * w;
            h -= query[i].1;
        }else if query[i].0 == 2{
            result = query[i].1 * h;
            w -= query[i].1;
        }
        println!("{}",result);
    }
    
}