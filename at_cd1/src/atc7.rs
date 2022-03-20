/// 実行方法
/// cargo run --bin atc7
/// https://atcoder.jp/contests/math-and-algorithm

use proconio::{ input, fastout };

fn main() {
    choose_card2_by_bit();
}

/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_t
#[allow(non_snake_case)]
fn choose_card2_by_dp() {
    input! {
        N: usize,
        A: [usize; N],
    }
    println!("{}", get_cnt_by_dp(A, N));
}

#[allow(non_snake_case)]
fn get_cnt_by_dp(A: Vec<usize>, N: usize) -> usize {
    const SELECT_CNT: usize = 5;
    const SUM_MAX: usize = 1000;
    let mut dp = vec![[0; SUM_MAX+1]; SELECT_CNT+1];
    // let mut dp = vec![[0; SELECT_CNT+1]; SUM_MAX+1];
    dp[0][0] = 1;

    // for n in 0..N {
    //     for i in 0..SUM_MAX {
    //         if i + A[n] > 1000 {
    //             continue;
    //         }
    //         for j in 0..SELECT_CNT {
    //             dp[i+A[n]][j+1] += dp[i][j];
    //         }
    //     }
    // }

    // dp[SUM_MAX][SELECT_CNT]

    // for n in 0..N {
    //     for i in (0..SUM_MAX).rev() {
    //         if i + A[n] > 1000{
    //             continue;
    //         }
    //         for j in (0..SELECT_CNT).rev() {
    //             dp[i+A[n]][j+1] += dp[i][j];
    //         }
    //     }
    // }

    // dp[SUM_MAX][SELECT_CNT]

// TODO rev()する理由確認

    for n in 0..N {
        for i in (0..SELECT_CNT).rev() {
            for j in (0..=SUM_MAX).rev() {
                // 既にチェックしてたら、continue
                // if dp[i+1][j] == 1{
                //     continue;
                // }
                // if j + A[n] > 1000{
                //     continue;
                // }
                
                if j + A[n] <= SUM_MAX {
                    dp[i+1][j+A[n]] += dp[i][j]; 
                }
            }
        }
    }

    dp[SELECT_CNT][SUM_MAX]
}

#[allow(non_snake_case)]
#[fastout]
fn choose_card2_by_bit() {
    input! {
        N: u128,
        A: [u128; N],
    }
    println!("{}", get_cnt_by_bit(A, N));
}

/// Nが大きくなるとbit足りないのでだめ
#[allow(non_snake_case)]
fn get_cnt_by_bit(A: Vec<u128>, N: u128) -> u128 {
    const SUM_MAX: u128 = 1000;
    let mut cnt = 0;
    for i in 0..(1 << N) {
        // println!("{:16b}", i);
        if (i as u128).count_ones() != 5 { continue; }
        let mut sum = 0;
        for j in 0..(N as usize) {
            if i & (1 << j) != 0 {
                sum += A[j];
            }
        }
        if sum == SUM_MAX {
            cnt += 1;
            // println!("{:16b}", i);
        }
    }
    cnt
}


/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_t
#[allow(non_snake_case)]
#[fastout]
fn choose_card2() {
    input! {
        N: u8,
        A: [u16; N],
    }
    println!("{}", get_cnt(A, N));
}

#[allow(non_snake_case)]
fn get_cnt(A: Vec<u16>, N: u8) -> u32 {
    const SUM_MAX: u16 = 1000;
    let mut cnt = 0;
    for i in 0..(N as usize) {
        if A[i] == SUM_MAX { continue; }
        for j in (i+1)..(N as usize) {
            if A[i] + A[j] >= SUM_MAX { continue; }
            for k in (j+1)..(N as usize) {
                if A[i] + A[j] +A[k] >= SUM_MAX { continue; }
                for l in (k+1)..(N as usize) {
                    if A[i] + A[j] + A[k] + A[l] >= SUM_MAX { continue; }
                    for m in (l+1)..(N as usize) {
                        let sum = A[i] + A[j] + A[k] + A[l] + A[m];
                        if sum == SUM_MAX {
                            cnt += 1;
                        }
                    }
                }
            }
        }
    }
    cnt
}



/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_s
#[fastout]
#[allow(non_snake_case)]
fn choose_card1() {
    input! {
        N: u32,
        A: [u8; N],
    }
    let (red_cnt, yellow_cnt, blue_cnt) = get_cnt_color(A);
    let ans: u64 = conv(red_cnt) + conv(yellow_cnt) + conv(blue_cnt);

    println!("{}", ans);
}

#[allow(non_snake_case)]
fn get_cnt_color(A: Vec<u8>) -> (u64, u64, u64) {
    let mut red_cnt: u64 = 0;
    let mut yellow_cnt = 0;
    let mut blue_cnt = 0;
    for a in A {
        match a {
            1 => red_cnt += 1,
            2 => yellow_cnt += 1,
            3 => blue_cnt += 1,
            _ => (),
        }
    }
    (red_cnt, yellow_cnt, blue_cnt)
}

fn conv(n: u64) -> u64 { n * (n-1) / 2 }

/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_r
#[allow(non_snake_case)]
#[fastout]
fn conv_store() {
    input! {
        N: u32,
        A: [u32; N],
    }

    let (cnt_100, cnt_200, cnt_300, cnt_400 ) = get_cnt_coin(A);
    let ans: u32 = (cnt_100 * cnt_400) + (cnt_200 * cnt_300);
    println!("{}", ans); 
}

#[allow(non_snake_case)]
fn get_cnt_coin(A: Vec<u32>) -> (u32, u32, u32, u32) {
    let mut cnt_100: u32 = 0;
    let mut cnt_200 = 0;
    let mut cnt_300 = 0;
    let mut cnt_400 = 0;
    for a in A {
        match a {
            100 => cnt_100 += 1,
            200 => cnt_200 += 1,
            300 => cnt_300 += 1,
            400 => cnt_400 += 1,
            _ => (),
        }
    }
    (cnt_100, cnt_200, cnt_300, cnt_400)
}



#[fastout]
#[allow(non_snake_case)]
fn least_comm_ints() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut ans_lcm = lcm(A[0], A[1]);
    for i in 2..N {
        ans_lcm = lcm(ans_lcm, A[i]);
    }
    println!("{}", ans_lcm);
}

fn lcm(a: usize, b: usize) -> usize {
    // (a * b) / gcd3(a, b) ← NG (a*b の値が非常に大きくなってオーバーフローしてしまう可能性があるのでNG)
    // a / gcd3(a, b) * b
    (a / gcd3(a, b)) * b
}

fn gcd3(a: usize, b: usize) -> usize {
    if b == 0 { return a; }
    gcd3(b, a % b)
}


/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_p
#[fastout]
#[allow(non_snake_case)]
fn greatest_n_ints() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut ans_gcd = gcd2(A[0], A[1]); 
    for i in 2..N {
        ans_gcd = gcd2(ans_gcd, A[i]);
    }
    println!("{}", ans_gcd);
}

fn gcd2(a: usize, b:usize) -> usize {
    if b == 0 { return a; }
    gcd2(b, a % b)
}

/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_o
#[allow(non_snake_case)]
#[fastout]
fn greatest_common_divisor() {
    input! {
        A: usize,
        B: usize,
    }
    println!("{}", gcd(A, B));
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { return a; }
    gcd(b, a % b)
}

/// テスト用
/// cargo build --bin atc7 && cargo test --bin atc7
#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    use std::process::Output;

    const BIN: &'static str = "./atc7"; // 実行ファイル名

    fn output(input_str: &str) -> Output {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(input_str)
            .tee_output()
            .expect_success();
        output
    }

    #[test]
    fn test_other() {
        assert_eq!(1+2, 3);
    }

    #[test]
    fn test1() {
        let input = r#"
            2   
        "#;
        let out = output(input);
        let exp: &str = &(r#"
          7
        "#
        .trim().split(" ").map(|s| s.replace(" ", "")).collect::<String>() + "\n");
        assert_eq!(out.stdout_str(), exp );
        assert!(out.stderr_str().is_empty());
    }
}