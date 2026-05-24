use proconio::input;

fn main(){
    input!{
        n: usize,
        m: usize,
        trade: [(i64, i64); m],
    }
    let mut get = vec![0; n + 1];
    

    loop {
        let mut flag = 0;
        for i in &trade {
            for j in &get {
                if i.0 == *j {
                    get.push(i.1);
                    flag = 1;
                }
            }
        }
        if flag == 0{
            break;
        }
    }


    
    get.sort();
    let mut prev = 0;
    let mut result = 0;
    for i in get{
        if i != prev{
            result += 1;
        }
        prev = i;
    }
    println!("{result}");
    
}