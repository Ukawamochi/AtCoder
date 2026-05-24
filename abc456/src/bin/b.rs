use proconio::input;

fn main(){
    input!{
        a: [i64; 6],
        b: [i64; 6],
        c: [i64; 6],
    }
    let mut result = 0.0;


    let mut ap = 4;
    let mut bp = 5;
    let mut cp = 6;


    let mut app = 0;
    let mut bpp = 0;
    let mut cpp = 0;
    //数え上げる。
    for i in 0..6{
        if ap == a[i]{
            app += 1;
        }
        if bp == b[i]{
            bpp += 1;
        }
        if cp == c[i]{
            cpp += 1;
        }
    }
    result += (app as f64 / 6.0) * (bpp as f64 / 6.0) * (cpp as f64 / 6.0);



    ap = 4;
    bp = 6;
    cp = 5;

    let mut app = 0;
    let mut bpp = 0;
    let mut cpp = 0;
    //数え上げる。
    for i in 0..6{
        if ap == a[i]{
            app += 1;
        }
        if bp == b[i]{
            bpp += 1;
        }
        if cp == c[i]{
            cpp += 1;
        }
    }
    result += (app as f64 / 6.0) * (bpp as f64 / 6.0) * (cpp as f64 / 6.0);



    ap = 5;
    bp = 6;
    cp = 4;
    
    let mut app = 0;
    let mut bpp = 0;
    let mut cpp = 0;
    //数え上げる。
    for i in 0..6{
        if ap == a[i]{
            app += 1;
        }
        if bp == b[i]{
            bpp += 1;
        }
        if cp == c[i]{
            cpp += 1;
        }
    }
    result += (app as f64 / 6.0) * (bpp as f64 / 6.0) * (cpp as f64 / 6.0);


    ap = 5;
    bp = 4;
    cp = 6;

    let mut app = 0;
    let mut bpp = 0;
    let mut cpp = 0;
    //数え上げる。
    for i in 0..6{
        if ap == a[i]{
            app += 1;
        }
        if bp == b[i]{
            bpp += 1;
        }
        if cp == c[i]{
            cpp += 1;
        }
    }
    result += (app as f64 / 6.0) * (bpp as f64 / 6.0) * (cpp as f64 / 6.0);



    ap = 6;
    bp = 5;
    cp = 4;
    
    let mut app = 0;
    let mut bpp = 0;
    let mut cpp = 0;
    //数え上げる。
    for i in 0..6{
        if ap == a[i]{
            app += 1;
        }
        if bp == b[i]{
            bpp += 1;
        }
        if cp == c[i]{
            cpp += 1;
        }
    }
    result += (app as f64 / 6.0) * (bpp as f64 / 6.0) * (cpp as f64 / 6.0);



    ap = 6;
    bp = 4;
    cp = 5;
    
    let mut app = 0;
    let mut bpp = 0;
    let mut cpp = 0;
    //数え上げる。
    for i in 0..6{
        if ap == a[i]{
            app += 1;
        }
        if bp == b[i]{
            bpp += 1;
        }
        if cp == c[i]{
            cpp += 1;
        }
    }
    result += (app as f64 / 6.0) * (bpp as f64 / 6.0) * (cpp as f64 / 6.0);




    println!("{}",result);
    
}