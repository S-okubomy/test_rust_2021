use proconio::{ input, fastout };
use std::cmp::{ min, max };
use std::collections::{ HashMap };
use itertools::Itertools;

fn main() {
    mountain();
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn mountain() {
    input! {
        N: usize,
        mut ST: [(String, usize); N],
    }
    ST.sort_by(|a, b| b.1.cmp(&a.1));
    // println!("{:?}", ST);
    println!("{}", ST[1].0);
}


#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn tiny_arithmetic_sequence() {
    input! {
        A: [isize; 3],
    }
    for perm in A.iter().permutations(3) {
        // println!("{:?}", perm);
        if perm[2] - perm[1] == perm[1] - perm[0] {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn b_prob() {
    input! {
        S: String,
    }

    let rev_map: HashMap<String, String> = vec![("0", "0"), ("1", "1"), ("6", "9"), ("8", "8"), ("9", "6")]
        .iter().map(|v| (v.0.to_string(), v.1.to_string())).collect();
    // println!("{:?}", rev_map);
    let ans: String = S.chars().rev().map(|c| rev_map.get(&c.to_string()).unwrap().to_string()).collect();
    println!("{}", ans);
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn three_dice() {
    input! {
        x: [usize; 3],
    }
    // let ans: usize = x.iter().map(|v| 7-v).sum();
    let ans: usize = x.iter().map(|&v| 7-v).sum();
    println!("{}", ans);
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn at_coder_condominium() {
    input! {
        N: usize, K: usize,
    }

    let mut ans = 0;
    for i in 1..=N {
        for j in 1..=K {
            ans += format!("{}0{}",i,j).parse::<usize>().ok().unwrap();
        }
    }
    println!("{}", ans);
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn chinchirorin2() {
    input! {
        a: u8, b:u8, c: u8,
    }

    if a != b && a != c && b != c {
        println!("0");
    } else {
        // 排他的論理和： 同じものは0になって、違うものだけ残る
        // println!("2^2 = {}", 2^2); // → 0
        // println!("2^5^2 = {}", 2^5^2); // → 5
        // println!("2^2^5 = {}", 2^2^5); // → 5
        println!("{}", a^b^c);
    }
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn chinchirorin() {
    input! {
        a: u8, b:u8, c: u8,
    }

    if a == b {
        println!("{}", c);
    } else if a == c {
        println!("{}", b);
    } else if b == c {
        println!("{}", a);
    } else {
        println!("0");
    }
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn nuts2() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let sum: usize = A.into_iter().map(|a| {
        if a > 10 {
            a - 10
        } else {
            0
        }
    }).sum();
    println!("{}", sum);
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn nuts() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut cnt = 0;
    for a in A {
        if a > 10 {
            cnt += a - 10;
        }
    }
    println!("{}", cnt);
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn rock_paper_scissors() {
    input! {
        x: u8, y: u8,
    }

    let v: Vec<u8> = vec![0, 1, 2];
    if x == y {
        println!("{}", x);
    } else {
        let ans: u8 = v.into_iter().filter(|val| *val != x && *val != y).collect::<Vec<u8>>()[0];
        println!("{}", ans);
    }
} 

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn permutation_check() {
    input! {
        N: usize,
        mut A: [usize; N],
    }

    A.sort_by(|a, b| a.cmp(b));
    for i in 0..A.len() {
        if i+1 != A[i] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn kcal() {
    input! {
        A: f64, B: f64,
    }

    let ans: f64 = (B / 100_f64) * A;
    println!("{}", ans);
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn savings() {
    input! {
        N: usize,
    }

    let mut x = 0;
    let mut sum = 0;
    while sum < N {
        x += 1;
        sum = x *(1 + x) / 2;
    }
    println!("{}", x);
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn maxi_buying() {
    input! {
        N: usize,
    }

    let pr: usize = (N * 108) / 100; // 整数なので小数点以下は切り捨てられる
    // println!("{}", pr);
    if 206 > pr {
        println!("Yay!");
    } else if 206 == pr {
        println!("so-so");
    } else {
        println!(":(");
    }

    // 実数型は誤差でる。
    // let pr: f64 = (N * 1.08).floor();
    // println!("{}", pr);
    // if 206_f64 > pr {
    //     println!("Yay!");
    // } else if 206_f64 == pr {
    //     println!("so-so");
    // } else {
    //     println!(":(");
    // }
}

/// https://atcoder.jp/contests/abc207/tasks/abc207_b
#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn hydrate2() {
    input! {
        A: f64, B: f64, C: f64, D: f64,
    }

    // 成り立つ条件
    // A + B * X <= (C * X) * D 
    // A <= (C*D - B)*X
    // A / (C*D-B) <= X
    // 成り立たない条件
    // 0 >= CD-B
    if B / C >= D {
        println!("-1");
        return;
    }
    let ans = (A / (C * D - B)).ceil();
    println!("{}", ans);
}


#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn hydrate() {
    input! {
        A: f64, B: f64, C: f64, D: f64,
    }

    if B / C >= D {
        println!("-1");
        return;
    }

    let mut cnt = 0;
    let mut b_q = A;
    let mut r_q = 0_f64;
    while r_q == 0_f64 || b_q / r_q > D {
        b_q += B;
        r_q += C;
        cnt +=1;        
    }
    println!("{}", cnt);
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn repression() {
    input! {
        X: [u32; 3],
    }

    let mut max_val = 0;
    let x_len = X.len();
    for i in 0..x_len {
        for j in i+1..x_len {
            let wa = X[i] + X[j];
            max_val = max(max_val, wa);
        }
    }
    println!("{}", max_val);
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn factorial_yen_coin() {
    input! {
        mut P: usize,
    }
    let mut cnt = 0;
    for x in (1..=10).rev() {
        let coin_val = fact(x);
        while P >= coin_val {
            cnt += 1;
            P -= coin_val;
        }
    }
    println!("{}", cnt);
}

fn fact(x: usize) -> usize {
    match x {
        0 | 1 => 1,
        _ => x * fact(x -1),
    }
}


#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn rolling_dice() {
    input! {
        A: f64, B: f64,
    }

    if A <= B && B / A <= 6.0 {
        println!("Yes");
    } else {
        println!("No");
    }
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn bouzu_mekuri2() {
    input! {
        _N: usize,
        S: String,
    }
    let num_bad_pos: usize = S.find('1').unwrap();
    println!("{}", if num_bad_pos % 2 == 0 { "Takahashi" } else { "Aoki" });
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn bouzu_mekuri() {
    input! {
        _N: usize,
        S: String,
    }
    // let num_bad: usize = S.chars().map(|c| c.to_string().parse::<u8>().ok().unwrap())
    //     .enumerate().max_by(|a,b| (a.1).cmp(&b.1)).unwrap().0 + 1;

    let s_vec: Vec<String> = S.chars().map(|c| c.to_string()).collect();
    let mut cnt = 0;
    for s in s_vec {
        cnt += 1;
        if s == "1" {
            break;
        }
    }
    println!("{}", if cnt % 2 == 0 { "Aoki" } else { "Takahashi" });
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