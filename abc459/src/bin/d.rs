use proconio::input;

fn main(){
    input!{
        t: usize,
        case: [String; t],
    }

    for word in case {
        let mut list = [0; 27];
        for c in word.chars(){
            let index = (c as usize) - ('a' as usize ) + 1;
            list[index] += 1;
        }
        let mut max = 0;
        for i in 1..=26{
            if list[max] < list[i]{
                max = i;
            }
        }
        if word.len() + 1 / 2 >= list[max] {
            
        }
        
        
    }
    
}

