use proconio::input;

fn main(){
    input!{
        n: usize,
        m: usize,
        s: String,
        t: String,
        q: usize,
        w: [String; q],
    }
    let mut sarr = [0; 26];
    let mut tarr = [0; 26];
    let mut arr = [0; 26];
    for i in s.chars(){
        let index = i as u8 - b'a';
        sarr[index as usize] = 1;
    }
    for i in t.chars(){
        let index = i as u8 - b'a';
        tarr[index as usize] = 1;
    }
    for i in 0..26{
        if tarr[i] == 1 && sarr[i] == 0{
            arr[i] = 1;
        }else if tarr[i] == 0 && sarr[i] == 1{
            arr[i] = 2;
        }
    }
    for i in w{
        let mut length = i.len();
        for j in i.chars(){
            let result = j as u8 - b'a';
            if arr[result as usize] == 1{
                println!("Aoki");
                break;
            }else if arr[result as usize] == 2{
                println!("Takahashi");
                break;
            }
            length -= 1;
            if length == 0{
               println!("Unknown");
            }
        }
    }
    
}

