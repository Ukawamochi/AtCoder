use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        x: usize,
        y: usize,
        l: usize,
        r: usize,
        a: usize,
        b: usize,
    }
    let all = b - a;
    let mut count = 0;

    
    for i in a..b{
        if l <= i && i < r{
            count += 1;
        }
    }
    eprintln!("x: {count} * {x}");
    eprintln!("y: {} * {y}",all - count);
    println!("{}",(all - count) * y + count * x);
}
