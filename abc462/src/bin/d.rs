use proconio::input;
fn main() {
    input! {
        n: usize,
        d: usize,
    }
    let mut enter = Vec::new();
    let mut out = Vec::new();
    let mut max = 0;
    let mut result = 0;
    
    for i in 0..n{
        input!{
            s: usize,
            t: usize,
        }
        if t - s + 1> d{
            enter.push(s);
            out.push(t);
            if t > max{
                max = t;
            }
        }else{
            //eprintln!("reject {i}");
        }
    }
    
    for start in 0..max{
        let mut tmp = 0;
        for i in 0..enter.len(){
            if enter[i] <= start && start + d <= out[i]{
                tmp += 1;
            }
        }
        //eprintln!("start {start} tmp {tmp}");
        result += tmp * (tmp - 1) / 2;
    }

    println!("{result}");
}
