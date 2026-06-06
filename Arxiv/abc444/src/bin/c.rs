use proconio::input;

fn main(){
    input!{
        n: usize,
        mut a: [i64; n],
    }
    let sum :i64 = a.iter().sum();
    a.sort();
    let max_len = a[n - 2] + a[n - 1];
    let mut result = Vec::new();
    //Lの長さ候補を見つけ、2つ選んでLになるかを確かめる。
    for i in 1..=(2*n){
        //i本で割り切れたとき一本の長さはsum/i=lになる
        if sum % (i as i64) == 0 && sum / (i as i64) <= max_len {
            //1本の長さはl
            let l = sum / (i as i64);
            
            
            let mut list = a.clone();
            //折れていない棒をlistから除く。
            for j in 0..list.len(){
                if list[j] == l{
                    list[j] = 0;
                }
            }
            
            
            list.sort();
            let mut right = list.len() - 1;
            let mut left = 0;
            
            loop{
                if right <= left{
                    if right == 0 {break; }
                    break;
                }
                
                if list[right] == 0{
                    if right == 0 {break; }
                    right -= 1;
                    continue;
                }
                if list[left] == 0{
                    left += 1;
                    continue;
                }
                if list[left] + list[right] == l{
                    list[left] = 0;
                    list[right] = 0;
                }else if list[left] + list[right] > l{
                    if right == 0 {break; }
                    right -= 1;
                }else if list[left] + list[right] < l{
                    left += 1;
                }
                
            }
            
            let mut complete = 1;
            
            for m in 0..list.len(){
                if list[m] != 0{
                    complete = 0;
                }
            }
            //すべて0にできたらpushしてこのlのループを終わらせる
            if complete == 0{
                continue;
            }else if complete == 1{
                result.push(l);
            }
            
            
            
        }
    }
    //昇順でソート
    result.sort();
    //結果表示
    for i in 0..result.len(){
        if i > 0{
            print!(" ");
        }
        print!("{}",result[i]);
    }
    println!();
}