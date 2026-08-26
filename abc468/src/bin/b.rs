use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        m: usize,
        d: usize,
        s: Chars,
    }
    let mut guard = s.clone();
    for i in 0..m{
        if s[i] == 'G'{
            for j in 1..=d{
                if i < j{
                    break;
                }
                guard[i - j] = 'G';
            }
            for j in 1..=d{
                if i + j >= m{
                    break;
                }
                guard[i + j] = 'G';
            }
        }
    }
    let mut count = 0;
    for i in guard {
        if i == '.'{
            count+= 1;
        }
    }
    println!("{count}");
}
