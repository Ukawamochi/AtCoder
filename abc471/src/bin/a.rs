use proconio::input;
fn main() {
    input! {
        a: f64,
        b: f64,
    }
    eprintln!("{a},{b}");
    if a + b == 9.0 || a - b == 9.0 || a / b == 9.0 || a * b == 9.0 {
        eprintln!("{} {} {} {}",a + b,a - b, a / b,a * b);
        println!("Nine");
    }else{
        println!("Nein");
    }
}
