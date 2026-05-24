use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        h: i64,
        w: i64,
        s: [Chars; h],
    }
    let mut result = 0;
    for wl in 0..w{
        for wr in wl..w{
            for ht in 0..h{
                for hb in ht..h{
                    //println!("{},{},{},{}",wl,wr,ht,hb);
                    let issym = check(&s,wl,wr,ht,hb);
                    if issym == true{
                        result += 1;
                    }
                    
                    
                }
            }
        }
    }    
    
    println!("{}",result);
    
}
fn check(s: &Vec<Vec<char>>, wl: i64, wr: i64, ht: i64, hb: i64) -> bool{
    
    for j in 0..(hb - ht + 1){
        for i in 0..(wr - wl + 1){
            if s[(hb - j) as usize][(wl + i) as usize] != s[(ht + j) as usize][(wr - i) as usize]{
                //println!("falase");
                return false
            }
        }
    }
    //0行目だけで対象担っているかどうか検証する
    //println!("true");
    return true
}