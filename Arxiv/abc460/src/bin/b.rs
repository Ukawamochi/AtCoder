use proconio::input;

fn main(){
    input!{
        t: i64,
    }
    for _i in 0..t{
        input!{
            x1: i64,
            y1: i64,
            r1: i64,
            x2: i64,
            y2: i64,
            r2: i64,
        }
        let dis = (x1 - x2)*(x1 - x2) + (y1 - y2) * (y1 - y2);
        let rr = (r1 + r2) * (r1 + r2);
        let rrd = (r1 - r2) * (r1 - r2);
        if rrd <= dis && dis <= rr {
            println!("Yes");
        }else{
            println!("No");
        }
        
    }
    
}

