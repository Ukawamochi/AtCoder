use proconio::input;

fn main(){
    input!{
        n: usize,
        s: [String; n],
    }
    for i in s{
        let first = i.chars().nth(0).unwrap();
        if first == 'a' || first == 'b' || first == 'c' {
            print!("2");
        }else if first == 'd' || first == 'e' || first == 'f' {
            print!("3");
        }else if first == 'g' || first == 'h' || first == 'i' {
            print!("4");
        }else if first == 'j' || first == 'k' || first == 'l' {
            print!("5");
        }else if first == 'm' || first == 'n' || first == 'o' {
            print!("6");
        }
        else if first == 'p' || first == 'q' || first == 'r' || first == 's'{
            print!("7");
        }
        else if first == 't' || first == 'u' || first == 'v' {
            print!("8");
        }else if first == 'w' || first == 'x' || first == 'y' || first == 'z' {
            print!("9");
        }
        
    }
    println!("");
    
    
    
}