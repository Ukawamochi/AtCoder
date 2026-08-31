use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        sin: Chars,
        tin: Chars,
    }
    let mut s = Vec::new();
    let mut t = Vec::new();
    for i in 0..n{
        let tmp = sin[i].to_digit(10).unwrap();
        s.push(tmp);
    }
    for i in 0..m{
        let tmp = tin[i].to_digit(10).unwrap();
        t.push(tmp);
    }
    let end = n - m;
    
    let mut min = 0;
    for i in 0..m{
        eprintln!("diff {}",t[i] - s[i]);
        if t[i] <= s[i]{
            min += s[i] - t[i];
        }else{
            min += 10 + s[i] - t[i];
        }
        eprintln!("{min}");
    }
    eprintln!("set min {min}");

    eprintln!("end: {end}");
    for start in 0..=end{
        let mut count = 0;
        for i in 0..m{
            if t[i] <= s[start + i]{
                count += s[start + i] - t[i];
            }else{
                count += 10 + s[start + i] - t[i];
            }
        }
        eprintln!("start: {start}, count = {count}");
        if count < min{
            min = count;
        }
    }
    println!("{min}");
    
    
}
