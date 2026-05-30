use proconio::input;

fn main(){
    input!{
        h: usize,
        w: usize,
        mut tmp: [String; h],
    }
    let mut arr = vec![vec![2; w + 3]; h + 3];
    let mut wx;
    let mut hx = 1;
    for i in tmp{
        wx = 1;
        for j in i.chars(){
            if j == '#' {
                arr[hx][wx] = 1;
            }else if j == '.' {
                arr[hx][wx] = 0;
            }
            wx += 1;
        }
        hx += 1;
    }

    let target = arr[1][1];
    let mut same = 0;
    for i in 1..=h{
        for j in 1..=w{
            if target != arr[i][j]{
                same = 1;
            }
        }
    }
    if same == 0{
        for i in 1..=h{
            for j in 1..=w{
                if arr[i][j] == 0{
                    print!(".");
                }else if arr[i][j] == 1{
                    print!(".");
                }
                if j == w{
                    println!();
                }
            }
        }
        
        return
    }

    
    let mut even = 0;
    loop{
        for i in 1..=h{
            for j in 1..=w{
                if arr[i][j] == 0{
                    if arr[i + 1][j] == 1 || arr[i + 1][j + 1] == 1 || arr[i][j + 1] == 1 || arr[i - 1][j + 1] == 1 || arr[i - 1][j] == 1 || arr[i - 1][j - 1] == 1 || arr[i][j - 1] == 1 || arr[i + 1][j - 1] == 1 {
                        arr[i][j] = 3;
                    }
                }
            }
        }
        for i in 1..=h{
            for j in 1..=w{
                if arr[i][j] == 3{
                    arr[i][j] = 1;
                }else if arr[i][j] == 1{
                    arr[i][j] = 0;
                }
            }
        }
        even = (even + 1) % 2;
        let mut flag = 0;
        let mut tar;
        for i in 1..=h{
            for j in 1..=w{
                tar = arr[i][j];
                if (arr[i + 1][j] == tar || arr[i + 1][j] == 2)
                && (arr[i + 1][j + 1] == tar || arr[i + 1][j + 1] == 2)
                && (arr[i][j + 1] == tar || arr[i][j + 1] == 2)
                && (arr[i - 1][j + 1] == tar  || arr[i - 1][j + 1] == 2)
                && (arr[i - 1][j] == tar || arr[i - 1][j] == 2)
                && (arr[i - 1][j - 1] == tar || arr[i - 1][j - 1] == 2) 
                && (arr[i][j - 1] == tar || arr[i][j - 1] == 2)
                && (arr[i + 1][j - 1] == tar || arr[i + 1][j - 1] == 2) {
                    flag = 1;
                }
            }
        }
        if flag == 0 && even % 2 == 0{
            break;
        }
    }

    
    for i in 1..=h{
        for j in 1..=w{
            if arr[i][j] == 0{
                print!(".");
            }else if arr[i][j] == 1{
                print!("#");
            }
            if j == w{
                println!();
            }
        }
    }
    
    
}

