/// 実行方法
/// cargo run --bin atc3
/// https://atcoder.jp/contests/math-and-algorithm
/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_g

use proconio::{ input, fastout };

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: isize,
        X: isize,
        Y: isize,
    }
    // XとYの倍数の合計個数から被ってる個数（最小公倍数）を引く
    println!("{}", N/X + N/Y - N/lcm(X, Y));
}

// 最小公倍数を求める
fn lcm(a: isize, b: isize) -> isize {
    (a * b) / gcd(a, b)
}

// 最大公約数
fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        return a;
    }
    gcd(b, a%b)
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