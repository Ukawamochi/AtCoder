use proconio::input;

fn main(){
    input!{
        n: usize,
        m: usize,
        //Cはコショウの在庫
        mut c: [i64; m],
        //Aは種類、Bは総量
        ab: [(i64, i64); n],
    }
    let mut count = 0;
    for i in 0..n{
        let stock = c[ab[i].0 as usize - 1];
        if stock >= ab[i].1{
            //在庫を減らす
            c[ab[i].0 as usize - 1] -= ab[i].1;
            //これまでかけた量を増やす
            count += ab[i].1;
        }else{
            //残りを足してストックを0にする
            count += stock;
            c[ab[i].0 as usize - 1] = 0;
        }        
    }
    println!("{}",count);
    
}