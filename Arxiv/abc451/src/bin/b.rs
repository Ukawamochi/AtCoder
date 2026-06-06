use proconio::input;

fn main(){
    input!{
        n: usize,
        m: i64,
        a: [(i64, i64); n],
    }
    let mut part = vec![0; 1 + m as usize];
    let mut next_part = vec![0; 1 + m as usize];
    
    for i in 0..n{
        //println!("part:{} is {}",a[i as usize].0, part[a[i as usize].0 as usize]);
        //println!("next part:{} is {}",a[i as usize].1, next_part[a[i as usize].1 as usize]);
        part[a[i as usize].0 as usize] += 1;
        next_part[a[i as usize].1 as usize] += 1;
    }
    
    for i in 1..=m{
        let result = next_part[i as usize] - part[i as usize];
        println!("{}",result);
    }
    
    
    
}