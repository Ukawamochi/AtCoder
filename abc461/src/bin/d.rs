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
    for h1 in 0..h{
        for w1 in 0..w{
            for h2 in h1..h{
                for w2 in w1..w{
                    if k == count_sum(&cum, h1 + 1, w1 + 1, h2 + 1, w2 + 1) {
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