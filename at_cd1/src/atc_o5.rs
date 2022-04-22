use proconio::{ input, fastout };

fn main() {
    cabbages();
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn cabbages() {
    input! {
        N: usize, A: usize, X: usize, Y: usize,
    }
    let mut ans = X * N;
    if N > A {
        ans = X * A + (N - A) * Y;
    }
    println!("{}", ans);
}