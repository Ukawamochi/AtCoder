use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
fn main(){
    input!{
        x: i64,
        q: usize,
    }
    
    let mut u50: BinaryHeap<i64> = BinaryHeap::new();
    let mut t50: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    u50.push(x);
    let mut middle = *u50.peek().unwrap();

    for _i in 0..q{
        input!{
            at: i64,
            bt: i64,
        }
        let mut under = 0;
        
        if at <= middle{
            u50.push(at);
            under += 1;
        }else{
            t50.push(Reverse(at));
        }
        if bt <= middle{
            u50.push(bt);
            under += 1;
        }else{
            t50.push(Reverse(bt));
        }

        //片方のヒープに2つ入ったら移動させる。
        if under == 2{
            let trans = u50.pop().unwrap();
            t50.push(Reverse(trans));
        }else if under == 0{
            let Reverse(trans) = t50.pop().unwrap();
            u50.push(trans);
        }
        
        middle = *u50.peek().unwrap();
        println!("{}",middle);
        
    }
}
