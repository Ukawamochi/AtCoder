use proconio::input;

fn main(){
    input!{
        n: usize,
        k: i64,
        mut a: [i64; n],
    }
    
    a.sort();
    let mut sum_arr: Vec<(i64, i64)> = Vec::new();
    let mut target = a[0];
    let mut count = 0;
    sum_arr.push((count as i64, a[0]));
    //println!("new {} ,{}",count, a[0]);
    //println!("{}",sum_arr[count].0);
    for i in 1..n{
        if target != a[i]{
            target = a[i];
            count += 1;
            sum_arr.push((count as i64, a[i]));
            //println!("new {} ,{}",count, a[i]);
        }else{ 
            sum_arr[count].1  += a[i];
            //println!("loop: {} add {} ,{}",i, count, sum_arr[count].1);
        }
    }
    
    //総和の大きさを降順でソート
    sum_arr.sort_by(|a, b| b.1.cmp(&a.1));
    
    //消すといい数字を探すために数字ごとに総和を求める。
    // その後sortして大きいやつを消す。
    for i in 0..k{
        if i > count as i64{
            break;
        }
        sum_arr[i as usize].1 = 0;
    }
    
    
    let mut sum = 0;
    for i in 0..sum_arr.len(){
        sum += sum_arr[i].1;
    }
    println!("{}", sum);
}