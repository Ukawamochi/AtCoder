use proconio::input;

fn main(){
    input!{
        h: i64,
        w: i64,
    }
    for i in 0..h{
        if i == 0 || i == (h - 1) {
            for _i in 0..w{
                print!("#");
            }
            println!("");
        }else{
            for j in 0..w{
                if j == 0 || j == (w - 1) {
                    print!("#");
                }else{
                    print!(".");
                }
            }
            println!("");
        }
    }
    
}