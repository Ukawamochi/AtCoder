use proconio::input;

fn main(){
    input!{
        d: f64,
    }
    //let pi = 3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117;
    let pi = std::f64::consts::PI;
    let s = d * d * pi / 4.0;
    println!("{}",s);
    
}