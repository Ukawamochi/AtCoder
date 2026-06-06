use proconio::input;
fn main() {
    input!{
        n: usize,
        data: [(i32, i32, i32); n],        
    }
    let mut flag = 0;
    let mut locate = (0, 0, 0);
    
    for i in 0..n{
        let required_time = (locate.0 - data[i].0).abs();
        let x = (locate.1 - data[i].1).abs();
        let y = (locate.2 - data[i].2).abs();
        
        let actual_time = x + y;
        
        if actual_time > required_time || (actual_time - required_time) % 2 != 0{
            flag = 1;
            break;  
        }
        
        
        locate = (data[i].0, data[i].1, data[i].2);
    }
    if flag == 1{
        println!("No");
    }else if flag == 0{
        println!("Yes");
    }
    
    
}
