use proconio::input;
fn main() {
    input! {
        d: usize,
        mut f: usize,
    }
    loop{
        if f > d{
            f -= d;
            println!("{f}");
            break;
        }
        f += 7;
    }
}
