/// 実行方法
/// cargo run --bin atc6
/// https://atcoder.jp/contests/math-and-algorithm

use proconio::{ input, fastout };

#[allow(non_snake_case)]
#[fastout]
fn main() {
    factorial();
}

/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_i
#[allow(non_snake_case)]
#[fastout]
fn factorial() {
    // TODO 実装
    println!("TODO 実装する");
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