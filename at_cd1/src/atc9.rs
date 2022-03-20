use proconio::{ input, fastout };
use std::cmp:: { min, max };

fn main() {
    taros_vacation();
}

/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ac
#[allow(dead_code)]
#[allow(non_snake_case)]
#[fastout]
fn taros_vacation() {
    input! {
        N: usize,
        mut A: [u64; N],
    }

    A.insert(0, 0);

    // dp1[i]: i日目に勉強する場合の、これまでの実力アップの最大値
    let mut dp1: Vec<u64> = vec![0; N+1];
    // dp2[i]: i日目に勉強しない場合の、これまでの実力アップの最大値
    let mut dp2: Vec<u64> = vec![0; N+1];

    dp1[0] = 0;
    dp2[0] = 0;
    for i in 1..=N {
        dp1[i] = dp2[i-1] + A[i];
        dp2[i] = max(dp1[i-1], dp2[i-1]);
    }

    println!("{}", max(dp1[N], dp2[N]));
}



/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_i
#[allow(dead_code)]
#[fastout]
#[allow(non_snake_case)]
fn brute_force_2() {
    input! {
        N: usize,
        S: usize,
        mut A: [usize; N],
    }
    A.insert(0, 0);

    // dp[i][j]: カードi番目までの中から、合計がjとなるかどうか
    let mut dp: Vec<Vec<bool>> = vec![vec![false; S+1]; N+1];
    dp[0][0] = true;

    for i in 1..=N {
        let card_no = A[i]; 
        for j in 0..=S {
            if card_no <= j {
                if dp[i-1][j] || dp[i-1][j-card_no] {
                    dp[i][j] = true;
                } else {
                    dp[i][j] = false;
                }
            } else {
                dp[i][j] = dp[i-1][j];
            }
        }
    }
    println!("{}", yes_or_no(dp[N][S]));
}

fn yes_or_no(b: bool) -> String {
    if b {
        String::from("Yes")
    } else {
        String::from("No")
    }
}


/// https://atcoder.jp/contests/math-and-algorithm/tasks/dp_d
#[allow(dead_code)]
#[allow(non_snake_case)]
#[fastout]
fn k_napsack1() {
    input! {
        N: usize,
        W: usize,
        mut wv: [(usize, usize); N],
    }
    wv.insert(0, (0, 0)); // index=1から計算始めれるように0番目追加

    // dp[i][wg]: 品物iまでの中から、重さ合計がwgとなるように選んだ時の最大価値
    let mut dp: Vec<Vec<usize>> = vec![vec![0; W+1]; N+1];
    dp[0][0] = 0;
    for i in 1..=N {
        let (wg, val) = wv[i];
        for w in 0..=W {
            if wg <= w {
                dp[i][w] = max(dp[i-1][w], dp[i-1][w-wg] + val);
            } else {
                dp[i][w] = dp[i-1][w];
            }
        }
    }

    // for i in 0..=N { println!("{:2?}", dp[i]) };
    println!("{}", dp[N][W]);
}


/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ab
#[allow(dead_code)]
#[allow(non_snake_case)]
#[fastout]
fn climb_stairs() {
    input! {
        N: usize,
    }

    let mut dp: Vec<usize> = vec![0; N+1];
    dp[0] = 1;
    dp[1] = 1;
    for i in 2..=N {
        dp[i] = dp[i-1] + dp[i-2];
    }

    println!("{}", dp[N]);
}


/// https://atcoder.jp/contests/math-and-algorithm/tasks/dp_a
#[allow(dead_code)]
#[allow(non_snake_case)]
#[fastout]
fn frog1_1() {
    input! {
        N: usize,
        mut h: [i32; N],
    }
    h.insert(0, 0); // 分かり易いようindex=1から計算できるよう0番目を追加

    let mut dp: Vec<i32> = vec![0; N+1];
    dp[1] = 0;
    dp[2] = (h[2] - h[1]).abs();

    for i in 3..=N {
        let v1 = dp[i-1] + (h[i] - h[i-1]).abs();
        let v2 = dp[i-2] + (h[i] - h[i-2]).abs();
        dp[i] = min(v1, v2);
    }

    println!("{}", dp[N]);
}


/// https://atcoder.jp/contests/math-and-algorithm/tasks/dp_a
#[allow(dead_code)]
#[fastout]
#[allow(non_snake_case)]
fn frog1() {
    input! {
        N: usize,
        h: [i32; N],
    }

    let mut dp: Vec<i32> = vec![0; N];
    dp[0] = 0;
    dp[1] = (h[1] - h[0]).abs();
    for i in 2..N {
        let v1 = dp[i-1] + (h[i] - h[i-1]).abs();
        let v2 = dp[i-2] + (h[i] - h[i-2]).abs();
        dp[i] = min(v1, v2);
    }

    // for i in 0..N {
    //     println!("dp: {}", dp[i]);
    // }

    println!("{}", dp[N-1]);
}