use proconio::input;
use proconio::marker::Chars;

fn main(){
    input!{
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    }
    let mut result = 0;
    
    let grid: Vec<Vec<usize>> = s.iter()
        .map(|row| row.iter().map(|&c| if c == '1' { 1 } else { 0 }).collect())
        .collect();
    
    let cum = build_cum(&grid);
    
    //長方形の長さを決める
    for height in 1..=h{
        for width in 1..=w{
            //左上を決める
            //eprintln!("SIZE OF heiht:{height}, width:{width}");
            if height * width < k{
                continue
            }
            for start_h in 0..=(h - height){
                for start_w in 0..=(w - width){
                    if k == count_sum(&cum, start_h+1, start_w+1, start_h + height, start_w + width) {
                        result += 1;
                    }
                }
            }
        }
    }

    println!("{result}");
    
}

fn build_cum(grid: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let h = grid.len();
    let w = grid[0].len();
    let mut cum = vec![vec![0usize; w + 1]; h + 1];
    for i in 1..=h {
        for j in 1..=w {
            cum[i][j] = grid[i-1][j-1] + cum[i-1][j] + cum[i][j-1] - cum[i-1][j-1];
        }
    }
    cum
}

fn count_sum(cum: &Vec<Vec<usize>>, h1: usize, w1: usize, h2: usize, w2: usize) -> usize {
    cum[h2][w2] + cum[h1-1][w1-1] - cum[h1-1][w2] - cum[h2][w1-1]
}