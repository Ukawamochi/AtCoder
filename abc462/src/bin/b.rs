use proconio::input;
fn main() {
    input! {
        n: usize,
    }
    let mut send = vec![0; n];
    let mut send_man = vec![vec![0; n]; n];
    for i in 0..n{
        input!{
            k: usize,
            a: [usize; k],
        }
        for j in a{
            send[j - 1] += 1;
            send_man[j - 1][i] = 1;
        }
        
    }
    for i in 0..n{
        print!("{}",send[i]);
        let mut last = -1;
        for j in 0..send_man[i].len() {
            if send_man[i][j] == 1{
                last = j as isize;
            }
        }
        if last == -1{
            println!();
            continue
        }else{
            print!(" ");
        }
        for j in 0..send_man[i].len() {
            if send_man[i][j] == 1{
                let out = j + 1;
                print!("{}",out);
                if j as isize == last{
                    println!();
                }else{
                    print!(" ");
                }
            }
        }
    }
}
