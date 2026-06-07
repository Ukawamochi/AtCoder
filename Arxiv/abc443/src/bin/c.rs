use proconio::input;

fn main(){
    input!{
        n: usize,
        t: usize,
        a: [usize; n],
    }
    let mut time = 0;
    let mut open_time :usize = 0;
    if n == 0{
        println!("{}",t);
        return
    }
    for i in 0..(n-1){
        eprintln!("chack a[{}]={}",i,a[i]);
        if a[i] > open_time {
            eprintln!("time={}",time);
            eprintln!("time += {}",a[i] - open_time);
            time += a[i] - open_time;
            open_time = a[i] as usize + 100;
        }
        eprintln!("end check");
    }
    eprintln!("last check a[{}] = {}",n - 1, a[n - 1]);
    let last_check = a[n - 1] as usize;
    if last_check > open_time {
        time +=  last_check - open_time;
        eprintln!("time += {}",last_check - open_time);
        open_time = last_check + 100;
        eprintln!("open time {}",open_time);
    }
    eprintln!("t={}",t);
    eprintln!("open time={}",open_time);
    if t > open_time && t - open_time > 0{
        time += t - open_time;
    }
    println!("{time}");
    
}

