use proconio::input;
fn main() {
    input! {
        n: usize,
        query: usize,
        mut c: [usize;n],
    }
    let mut array = c.clone();
    for _i in 0..query{
        input!{
            q: usize,
        }
        if q == 1{
            input!{
                x: usize,
                y: usize,
            }
            let tmp = c[x - 1];
            c[x - 1] = c[y - 1];
            c[y - 1] = tmp;
        }else if q == 2{
            eprintln!("q2");
            for j in 0..n{
                eprintln!("j: {}",j);
                eprintln!("{}",array[c[j] - 1]);
                array[c[j] - 1] = j;
            }
            c = array.clone();
        }
    }
    print!("{}" ,array[0]);
    for i in 1..n{
        print!(" {}",array[i]);
    }
    println!();
}
