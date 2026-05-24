use proconio::input;

fn main(){
    input!{
        s: String,
    }
    let mut counter = [0; 26];
    for i in s.chars() {
        counter[(i as usize ) - ('a' as usize)] += 1;
    }
    let mut max = 0;
    for i in 0..26{
        if max < counter[i]{
            max = counter[i];
        }
    }
    let mut result= String::new();
    let mut flag = 0;
    for i in s.chars(){
        let target_index = (i as usize) - ('a' as usize);
        if counter[target_index] != max{
            result.push(i);
            flag = 1;
        }
    }
    if flag == 1{
        println!("{}",result);
    }
     
}