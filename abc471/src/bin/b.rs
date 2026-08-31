use proconio::input;
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut name = Vec::new();
    let mut count = Vec::new();
    let mut len = 0;
    for i in 0..n{
        let mut flag = 0;
        let mut index = 0;
        for j in 0..len{
            if s[i].to_uppercase() == name[j]{
                index = j;
                flag = 1;
            }
        }
        if flag == 0{
            name.push(s[i].to_uppercase());
            count.push(0);
            index = len;
            len+= 1;
        }
        count[index] += 1;
    }
    let mut max = count[0];
    for i in 0..len{
        eprintln!("{}",count[i]);
        if count[i] > max{
            max = count[i];
        }
    }
    println!("{max}");
}
