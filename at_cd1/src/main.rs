use proconio::input;

/// 実行方法
/// cargo run --bin main
#[allow(non_snake_case)]
fn main() {
   input!{
        N: usize,
        mut pos:[(i32, i32, i32); N],
    }
    let mut T_X_Y: Vec<(i32, i32, i32)> = vec![(0, 0, 0)];
    
    T_X_Y.append(&mut pos); // 初期状態

    if can_travel(N, T_X_Y) {
        println!("Yes");
    } else {
        println!("No");
    }
}

#[allow(non_snake_case)]
fn can_travel(N: usize, T_X_Y: Vec<(i32, i32, i32)>) -> bool {
    for i in 0..N {
        let (ti, xi, yi) = T_X_Y[i];
        let (ti_p1, xi_p1, yi_p1) = T_X_Y[i+1];
        let dt = ti_p1 - ti;
        let dist = (xi_p1 - xi).abs() + (yi_p1 - yi).abs();
        if dt < dist || dt%2 != dist%2 {
             return false; 
        }
    }
    true
}


/// テスト用
/// cargo build --bin main && cargo test --bin main
#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    use std::process::Output;

    const BIN: &'static str = "./at_cd1"; // 実行ファイル名

    #[test]
    fn test1() {        
        let input_str: &str = r#"
            2
            3 1 2
            6 1 1                  
        "#;
        let output: Output = output(input_str);
        assert_eq!(output.stdout_str(), "Yes\n");
        assert!(output.stderr_str().is_empty()); // 値がtrueならテスト成功
    }

    #[test]
    fn test2() {
        let input_str = r#"
            1
            2 100 100
        "#;
        let out = output(input_str);
        assert_eq!(out.stdout_str(), "No\n");
        assert!(out.stderr_str().is_empty());
    }

    #[test]
    fn test3() {
        let input = r#"
            2
            5 1 1
            100 1 1
        "#;
        let out = output(input);
        assert_eq!(out.stdout_str(), "No\n");
        assert!(out.stderr_str().is_empty());
    }

    #[test]
    fn test_other() {
        assert_eq!(1+2, 3);
    }

    fn output(input_str: &str) -> Output {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(input_str)
            .tee_output()
            .expect_success();
        output
    }
}