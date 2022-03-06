/// 実行方法
/// cargo run --bin atc4
/// https://atcoder.jp/contests/math-and-algorithm

use proconio::{ input, fastout };

#[allow(non_snake_case)]
#[fastout]
fn main() {
    brute_force_2_1();
}

/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_i
#[allow(non_snake_case)]
#[fastout]
fn brute_force_2_1() {
    input! {
        N: usize,
        S: usize,
        A: [usize; N],
    }
    let mut dp: Vec<Vec<bool>> = vec![vec![false; S+1]; N+1];

    dp[0][0] = true;

    for i in 0..N {
        if A[i] == S {
            println!("Yes");
            return;
        }

        for j in 0..=S {
            // 既にチェックしてたら、continue
            if dp[i+1][j] {
                continue;
            }

            dp[i+1][j] |= dp[i][j];

            // i番目のカード選ぶ場合:
            if  j >= A[i] {
                // N枚のカードから合計値がjになるように選んだ時、trueになるよう更新していく
                // (例) 2 5 9の場合: i=0, j=2の時、dp[i+1][j]=dp[1][2]=dp[i][j-A[i]]=dp[0][2-2]=dp[0][0]=true → カード値0と2を選択
                // (例) 2 5 9の場合: i=1, j=5の時、dp[i+1][j]=dp[2][5]=dp[i][j-A[i]]=dp[1][5-5]=dp[1][0]=true → カード値0と5を選択
                // (例) 2 5 9の場合: i=1, j=7の時、dp[i+1][j]=dp[2][7]=dp[i][j-A[i]]=dp[1][7-5]=dp[1][2]=true → カード値2と5を選択
                // (例) 2 5 9の場合: i=2, j=9の時、dp[i+1][j]=dp[3][9]=dp[i][j-A[i]]=dp[2][9-9]=dp[2][0]=true → カード値0と9を選択
                // (例) 2 5 9の場合: i=2, j=11の時、dp[i+1][j]=dp[3][11]=dp[i][j-A[i]]=dp[2][11-9]=dp[2][2]=true → カード値2と9を選択(dp[2][2]に以前選択した結果が格納されているから計算できる)
                dp[i+1][j] |= dp[i][j-A[i]];
            }
        }
    }

    // for i in 0..=N {
    //     for j in 0..=S {
    //         if dp[i][j] {
    //             println!("trueのindex dp[{}][{}]", i, j);
    //         }
    //     }
    // }

    println!("{}", yes_or_no(dp[N][S]));

// 縦軸i、横軸j
//   0 1 2 3 4 5 6 7 8 9 10 11
// 0 t   
// 1 t   t        
// 2 t   t     t   t       
// 3 t   t     t   t   t     t
}

fn yes_or_no(b: bool) -> String {
    if b {
        String::from("Yes")
    } else {
        String::from("No")
    }
}


/// テスト用
/// cargo build --bin atc5 && cargo test --bin atc5
#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    use std::process::Output;

    const BIN: &'static str = "./atc5"; // 実行ファイル名

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