/// 実行方法
/// cargo run --bin atc4
/// https://atcoder.jp/contests/math-and-algorithm
/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_h

use proconio::{ input, fastout };

#[allow(non_snake_case)]
#[fastout]
fn main() {
    brute_force_2();
}

// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_i
#[allow(non_snake_case)]
#[fastout]
fn brute_force_2() {
    input! {
        N: usize,
        S: usize,
        A: [usize; N],
    }
    let mut dp: Vec<Vec<bool>> = vec![vec![false; S+1]; N];

    if A[0] <= S {
        // i番目までのカードで合計がちょうどjになるように選ぶ場合にtrue設定される
        // （例）2 5 9の場合、dp[0][2]はtrue
        dp[0][A[0]] = true;
    }

    for i in 1..N {
        if A[i] == S {
            println!("Yes");
            return;
        }

        if A[i] <= S {
            // （例）2 5 9の場合、dp[1][5]はtrue
            dp[i][A[i]] = true;
        }

        for j in 1..=S {
            // （例）2 5 9の場合、dp[1][0]はfalse → not continue
            if dp[i][j] {
                continue;
            }

            // iが1つ前の結果を設定
            // （例）2 5 9の場合、dp[1][0] = dp[0][0] = false
            dp[i][j] = dp[i-1][j];

            // 選ぶ場合: j-A[i]
            // （例）2 5 9の場合、dp[i-1][j-A[i]]] = dp[1-1][5-A[1]] = dp[0][0] は、falseなので対象外
            // （例）2 5 9の場合、dp[i-1][j-A[i]]] = dp[1-1][7-A[1]] = dp[0][2] は、trueなので対象 
            if j >= A[i] && dp[i-1][j-A[i]] {
                // （例）2 5 9の場合、dp[1][7]にtrueを設定 
                dp[i][j] = true;
            }
        }
    }

    if dp[N-1][S] {
        println!("Yes");
    } else {
        println!("No");
    }

// 縦軸i、横軸j
//   0 1 2 3 4 5 6 7 8 9 10 11
// 0     t
// 1     t     t   t
// 2     t     t   t   t
// 3
}





/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_h
#[allow(non_snake_case)]
#[fastout]
fn brute_force_1_2() {
    input! {
        N: i32,
        S: i32,
    }

    let mut cnt = 0;
    for red_val in 1..=N {
        if red_val >= S {
            break;
        }
        for blue_val in 1..=N {
            if S >= red_val + blue_val {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}

/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_h
#[allow(non_snake_case)]
#[fastout]
fn brute_force_1() {
    input! {
        N: i32,
        S: i32,
    }

    let mut cnt: i32 = 0;
    for red_val in 1..=N {
        if red_val >= S {
            break;
        }

        let blue_val_cnt = S - red_val; // 赤カード値に対する青カードの書き込み方の数
        if N >= blue_val_cnt {
            cnt += blue_val_cnt;
        } else {
            cnt += N;
        }
    }

    println!("{}", cnt);
}


/// テスト用
/// cargo build --bin atc4 && cargo test --bin atc4
#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    use std::process::Output;

    const BIN: &'static str = "./atc3"; // 実行ファイル名

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