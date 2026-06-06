use proconio::input;
fn main() {
    input!{
        s: String,
    }
    let length = s.len();
    let mut dp = vec![false; length + 1];
    dp[0] = true;
    
    for i in 0..=length{
        if dp[i] == true{
            if i + 7 <= length && &s[i..i+7] == "dreamer"{
                dp[i+7] = true;
            }
            if i + 5 <= length && &s[i..i+5] == "dream"{
                dp[i+5] = true;
            }
            if i + 6 <= length && &s[i..i+6] == "eraser"{
                dp[i+6] = true;
            }
            if i + 5 <= length && &s[i..i+5] == "erase"{
                dp[i+5] = true;
            }
        }
    }
    if dp[length] == false{
        println!("NO");
    }else{
        println!("YES");
    }
}
