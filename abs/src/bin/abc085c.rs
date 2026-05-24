use proconio::input;
fn main() {
    input!{
        n: i32,
        yen: i32,
    }
    let mut x = -1;
    let mut y = -1;
    let mut z = -1;
    let mut flag = 0;
    for i in 0..=n{
        for j in 0..=n - i{
            let k = n - (i + j);
            if (yen == 10000 * i + 5000 * j + 1000 * k) && (i + j + k == n) {
                    x = i;
                    y = j;
                    z = k;
                    flag = 1;
                    break;
            }
        }
        if flag == 1{
            break;
        }
    }
    println!("{} {} {}",x,y,z);
}
