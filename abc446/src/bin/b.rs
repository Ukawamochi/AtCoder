use proconio::input;

fn main(){
    input!{
        n: usize,
        m: usize,
    }
    let mut list: Vec<Vec<i64>> = Vec::new();
    let mut stock = vec![1;m];
    for _i in 0..n{
        input!{
            l: usize,
            hope: [i64; l],
        }
        list.push(hope);
    }
    for i in 0..n{
        //println!("list[{}]:{}",i,list[i][0]);
    }
    
    for i in 0..n{
        for j in 0..list[i].len(){
            let want_juice_id = list[i][j] as usize - 1;
            if stock[want_juice_id] == 1{
                stock[want_juice_id] = 0;
                println!("{}",want_juice_id + 1);
                break;
            }
            if j == list[i].len() - 1{
                println!("0");
            }
        }
    }
    
}