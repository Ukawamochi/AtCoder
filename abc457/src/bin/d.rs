use proconio::input;

fn main(){
    input!{
        n: usize,
        mut k: usize,
    }
    let mut array = Vec::new();
    for i in 1..=(n as i64){
        input!{
            a: i64,
        }
        array.push((i, a));
    }
    if n == 1{
        println!("{}",array[0].1 + k as i64);
        return
    }
    array.sort_by(|a, b| a.1.partial_cmp(&(b.1)).unwrap());
    loop {
        if k == 0{
            break;
        }
        loop{
            array[0].1 += array[0].0;
            k -= 1;
            if k == 0{
                break;
            }
            if array[0].1 > array[1].1{
                //挿入ソートを実装する
                safe_insert_sort(&mut array);                
                break;
            }
        }
    }

    safe_insert_sort(&mut array);
    
    let min = array[0].1;
    println!("{}",min);


    fn safe_insert_sort<T: Ord>(arr: &mut [T]) {
        for i in 1..arr.len() {
            let mut j = i;
            while 0 < j && arr[j-1] > arr[j] {
                arr.swap(j-1, j);
                j -= 1;
            }
        }
    }
    
}