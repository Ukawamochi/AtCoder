    use proconio::input;
    
    fn main(){
        input!{
            n: usize,
        }
        
        let mut fee = vec![vec![-1i64; n]; n];
        for i in 0..n{
            input!{
                row: [i64; n - i - 1]
            }
            for j in 0..(n - i - 1){
                fee[i][j] = row[j];
                //print!("{},",fee[i][j]);
            }
            //println!("");
        }
        
        let mut flag = 0;
        for a in 0..n{
            for b in (a+1)..n{
                for c in (b+1)..n{
                    let atoc = fee[a][c - a - 1];
                    let atobtoc = fee[a][b - a - 1] + fee[b][c - b - 1];
                    //println!("a:{},b:{},c:{}",a,b,c);
                    //println!("atoc:{},atobtoc:{}",atoc,atobtoc);
                    if atoc > atobtoc{
                        flag = 1;
                    }
                }
            }
        }
        if flag == 1{
            println!("Yes");
        }else if flag == 0{
            println!("No");
        }
        
    } 