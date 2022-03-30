use proconio:: { input, fastout };
use std::collections::HashMap;

fn main() {
    b_mex(); 
}


/// https://atcoder.jp/contests/abc245/tasks/abc245_b
#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn b_mex() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut ans = 0;
    for num in 0..=2000 {
        if !A.iter().any(|a| { *a == num }) {
            ans = num;
            break;
        }
    }

    println!("{}", ans);
}


/// REになるのでNG
/// https://atcoder.jp/contests/abc245/tasks/abc245_b
#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn NG_b_mex2() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut hash_map = HashMap::new();
    for a in A {
        let cnt = hash_map.entry(a).or_insert(0);
        *cnt +=1;
    }

    let mut tmp_vec: Vec<_> = hash_map.iter().collect();
    tmp_vec.sort_by(|a, b| { (a.0).cmp(b.0) } ); // keyの値の小さい順

    // println!("{:?}", hash_map);
    // println!("{:?}", tmp_vec);

    let mut ans = 0;
    for i in 0..=2000 {
        let (key, _) = tmp_vec[i];
        if i != *key {
            ans = i;
            break;
        }
    }

    println!("{}", ans);
}



#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn good_morning() {
    input! {
        A: u8, B: u8, C: u8, D: u8, 
    }

    if A < C {
        println!("Takahashi");
    } else if A == C {
        if B == D {
            println!("Takahashi");
        } else {
            println!("Aoki");
        }
    } else {
        println!("Aoki");
    }
}