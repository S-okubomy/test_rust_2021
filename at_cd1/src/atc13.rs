use proconio::{ input, fastout };


fn main() {
    convi_store2();
}

/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_al
#[allow(non_snake_case)]
#[fastout]
fn convi_store2() {
    input! {
        T: usize,
        N: usize,
        mut LR: [(usize, usize); N],
    }
    LR.insert(0, (0, 0));

    // 階差
    let mut B: Vec<isize> = vec![0; T+1];
    B[0] = 0;
    for i in 1..=N {
        let (L, R) = LR[i];
        B[L] += 1;
        B[R] -= 1; // R-1までは居るけど、Rになると居ない 
    }

    // 累積和
    let mut S: Vec<isize> = vec![0; T];
    S[0] = B[0];
    for i in 1..=T-1 {
        S[i] = S[i-1] + B[i];
    }

    for i in 0..=T-1 {
        println!("{}", S[i]);
    }
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[fastout]
fn double() {
    input! {
        S: String,
    }

    let res = S.trim().parse::<i32>();
    let ans: String = match res {
        Ok(v) => (2 * v).to_string(),
        Err(_) => "error".to_owned(),
    };
    println!("{}", ans);

    // let res: Result<i32, &'static str> = match S.trim().parse::<i32>() {
    //     Ok(v) => Ok(2 * v),
    //     Err(_) => Err("error"),
    // };
    // match res {
    //     Ok(v) => println!("{}", v),
    //     Err(e) => println!("{}", e),
    // };
}

/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ak
#[allow(dead_code)]
#[allow(non_snake_case)]
#[fastout]
fn travel() {
    input! {
        N: usize,
        mut A: [usize; N-1],
        M: usize,
        mut B: [usize; M],
    }

    A.insert(0, 0);
    B.insert(0, 0);

    // 累積和
    let mut S: Vec<isize> = vec![0; N+1];
    S[1] = 0;
    for i in 2..=N {
        S[i] = S[i-1] + A[i-1] as isize;
    }

    let mut ans: isize = 0;
    for i in 1..=M-1 {
        let (prev_st, next_st) = (B[i], B[i+1]);
        ans += (S[next_st] - S[prev_st]).abs();
        // if next_st > prev_st {
        //     ans += S[next_st] - S[prev_st];
        // } else {
        //     ans += S[prev_st] - S[next_st];
        // }
    }

    println!("{}", ans);
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