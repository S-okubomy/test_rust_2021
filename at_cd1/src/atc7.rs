/// 実行方法
/// cargo run --bin atc7
/// https://atcoder.jp/contests/math-and-algorithm
fn main() {
    greatest_common_divisor();
}

/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_o
fn greatest_common_divisor() {
    // TODO 実装
    println!("TODO 実装!!!");
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