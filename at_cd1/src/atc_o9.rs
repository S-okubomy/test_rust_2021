use proconio::{ input };

fn main() {
    rotate();
}

#[allow(dead_code)]
fn rotate() {
    input! {
        s: String,
    }
    let c_vec: Vec<char> = s.chars().collect();
    let tmp_vec: Vec<char> = vec![c_vec[1], c_vec[2], c_vec[0]];
    let ans: String = tmp_vec.iter().collect();
    println!("{}", ans);
}

#[allow(dead_code)]
fn square_inequality() {
    input! {
        a: usize, b: usize, c: usize,
    }
    
    if a.pow(2) + b.pow(2) < c.pow(2) {
        println!("Yes");
    } else {
        println!("No");
    }
}