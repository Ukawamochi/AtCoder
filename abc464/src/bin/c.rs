use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut color = vec![0;n + 1];
    input!{
        mut adb: [(usize,usize,usize);n],
    }
    for i in 0..n{
        if adb[i].1 == 1{
            color[adb[i].2] += 1;
        }else{
            color[adb[i].0] += 1;
        }
    }
    let mut colors = 0;
    for i in &color{
        if *i >= 1{
            colors += 1;
        }
    }
    let mut array = vec![0;m + 1];
    array[1] = colors;
    
    adb.sort_by(|a, b| a.1.cmp(&b.1));//昇順でソート\
    
    for i in adb{
        if i.1 == 1{
            continue;
        }
        if i.0 == i.2 {
            continue;
        }
        if color[i.2] == 0{
            colors += 1;
        }
        if color[i.0] == 1{
            colors -= 1;
        }
        color[i.2] += 1;
        color[i.0] -= 1;
        array[i.1] = colors;
    }
    let mut last = array[1];
    for i in 1..=m{
        if array[i] == 0{
            println!("{last}");
        }else{
            last = array[i];
            println!("{}",array[i]);
        }
    }

}
