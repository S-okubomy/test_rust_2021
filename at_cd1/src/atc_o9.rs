use proconio::{ input };

fn main() {
    square_inequality();
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