use proconio::{ input };

fn main() {
    remove_it();
}

#[allow(dead_code)]
fn remove_it() {
    input! {
        n: usize, x: usize,
        a_vec: [usize; n],
    }
    let s_vec: Vec<String> = a_vec.into_iter().filter(|a| { *a != x }).map(|a| a.to_string()).collect::<Vec<String>>();
    println!("{}", s_vec.join(" "));
}

#[allow(dead_code)]
fn orthogonality() {
    input! {
        n: usize,
        a_vec: [isize; n],
        b_vec: [isize; n],
    }
    let mut naiseki = 0;
    for i in 0..n {
        naiseki += a_vec[i] * b_vec[i];
    }

    if naiseki == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
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