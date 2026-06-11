use proconio::input;

fn main(){
    input!{
        t: usize,
    }
    for _i in 0..t{
        input!{
            n: usize,
            w: usize,
            case: [usize; n]
        }
        let mut sum_arr = vec![0; 2 * w];
        let mut index = 1;
        for j in 0..n{
            sum_arr[index] += case[j];
            index += 1;
            if index == 2 * w{
                index = 0;
            }
        }
        
        let mut min = 0;
        let mut flag = 0;
        let mut remove_index = 0;
        let mut add_index = w;
        index = 0;
        let mut part = 0;
        
        for _h in 0..(2 * w){
            if flag == 0{
                for _j in 0..w{
                    if index == (2 * w){
                        index = 0;
                    }
                    part += sum_arr[index];
                    index += 1;
                }
                min = part;
                flag = 1;
            }else{
                if add_index == 2 * w{
                    add_index = 0;
                }
                if remove_index == 2 * w{
                    remove_index = 0;
                }
                part = part + sum_arr[add_index] - sum_arr[remove_index];
                add_index += 1;
                remove_index += 1;
            }
            //eprintln!("part = {part}");
            if part < min {
                //eprintln!("change min{part}");
                min = part;
            }
        }
        println!("{min}");
        //eprintln!("result{x} ---------");
        
    }
    
}

