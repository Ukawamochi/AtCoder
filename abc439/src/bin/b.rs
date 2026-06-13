use proconio::input;
fn main() {
    input! {
        mut n: usize,
    }
    let mut prev = Vec::new();
    let mut flag = 0;
    loop{
        let one = n % 10;
        let ten = n / 10 % 10;
        let hund = n / 100 % 10;
        let thou = n / 1000 % 10;
        n = one * one + ten * ten + hund * hund + thou * thou;
        if n == 1{
            println!("Yes");
            break;
        }
        for i in &prev{
            if i == &n{
                flag = 1;
                break;
            }
        }
        if flag == 1{
            println!("No");
            break;
        }
        prev.push(n);
    }
}
