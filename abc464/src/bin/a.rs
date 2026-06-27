use proconio::input;
fn main() {
    input! {
        s: String,
    }
    let mut e = 0;
    let mut w = 0;
    for i in s.chars(){
        if i == 'E'{
            e += 1;
        }else{
            w += 1;
        }
    }
    if e > w{
        println!("East");
    }else{
        println!("West");
    }
}
