use proconio::input;

fn main(){
    input!{
        n: usize,
        a: [i64; n],
    }
    let mut memory: Vec<i64> = Vec::new();
    let mut ans = vec![0; n];
    
    for i in 0..n{
        //iマスから始める。lastは止まった場所
        let mut last_addr = a[i] as usize - 1;
        memory.push(last_addr); 
        //10^100回移動させる(途中で必ずループするのでそれを探す。)
        for _j in 1..=n{
            //すでに求めていたら終了させる。
            if ans[a[last_addr] as usize - 1] != 0{
                print!("{}",ans[a[last_addr] as usize - 1]);
                if i != (n - 1){
                    print!(" ");
                }
                break;
            }
            //ループを検出したとき
            if last_addr == (a[last_addr] - 1)as usize{
                print!("{}",last_addr + 1);
                for k in memory.iter(){
                    if k == 1{
                        ans[k] = last_addr;
                    }
                }
                if i != (n - 1){
                    print!(" ");
                }
                break;
            }
            memory[(a[last_addr as usize] - 1) as usize] = 1;
            last_addr = a[last_addr] as usize - 1;
            
        }
        
    }
    println!();
}