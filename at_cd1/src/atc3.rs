/// 実行方法
/// cargo run --bin atc3
/// https://atcoder.jp/contests/math-and-algorithm

use proconio::input;

fn main() {
    input! {
        n: u8,
    }
    println!("{}", n + 5);
}

/// テスト用
/// cargo build --bin atc3 && cargo test --bin atc3
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