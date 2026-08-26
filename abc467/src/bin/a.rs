use proconio::input;
fn main() {
    input! {
        h: f64,
        w: f64,
    }
    eprintln!("h: {h}, w: {w}");
    let bmi = w / h / h * 10000.0 ;
    if bmi >= 25.0{
        println!("Yes");
    }else{
        println!("No");
    }
    eprintln!("{bmi}");
    
}
