use proconio::{ input, fastout };


fn main() {
    travel();
}

/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ak
fn travel() {
    println!("TODO");
}

/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_aj
#[allow(dead_code)]
#[allow(non_snake_case)]
#[fastout]
fn snowy_days() {
    input! {
        N: usize, Q: usize,
        mut LRX: [(usize, usize, usize); Q],
    }
    LRX.insert(0, (0, 0, 0));

    // 階差
    let mut B: Vec<isize> = vec![0; N+2];
    for i in 1..=Q {
        let (L, R, X) = LRX[i];
        B[L] += X as isize;
        B[R+1] -= X as isize;
    }

    for i in 2..=N {
        if 0 < B[i] {
            print!("<");
        } else if 0 == B[i] {
            print!("=");
        } else {
            print!(">");
        }
    }
}


/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ai
#[allow(dead_code)]
#[allow(non_snake_case)]
#[fastout]
fn how_many_guests() {
    input! {
        N: usize, Q: usize,
        mut A: [usize; N],
        mut LR: [(usize, usize); Q],
    }

    A.insert(0, 0);
    LR.insert(0, (0, 0));

    // 累積和
    let mut B: Vec<usize> = vec![0; N+1];
    for i in 1..=N {
        B[i] = B[i-1] + A[i];
    }
    
    for i in 1..=Q {
        let (L, R) = LR[i];
        let ans = B[R] - B[L-1];
        println!("{}", ans);
    }
}