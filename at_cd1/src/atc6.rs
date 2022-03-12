/// 実行方法
/// cargo run --bin atc6
/// https://atcoder.jp/contests/math-and-algorithm

use proconio::{ input, fastout };

#[allow(non_snake_case)]
#[fastout]
fn main() {
    factorization2();
}

/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_n
#[allow(non_snake_case)]
fn factorization2() {
    input! {
        mut N: usize, 
    }
    print_prime_factor(&mut N);
}

#[allow(non_snake_case)]
fn print_prime_factor(N: &mut usize) {
    let lim = (*N as f64).sqrt() as usize;
    for num in 2..=lim {
        while *N % num == 0 {
            print!("{} ", num);
            *N /= num;
        }
    }
    if *N >= 2 {
        println!("{}", N); // ※ここ通らないと改行されない
    }
}

#[allow(dead_code)]
#[allow(non_snake_case)]
fn factorization() {
    input! {
        mut N: usize,
    }
    let s = get_prime_factor(&mut N).iter().map(|n| n.to_string()).collect::<Vec<String>>().join(" ");
    println!("{}", s);
}

#[allow(dead_code)]
#[allow(non_snake_case)]
fn get_prime_factor(N: &mut usize) -> Vec<usize> {
    let lim = (*N as f64).sqrt() as usize;
    let mut fac_vec: Vec<usize> = Vec::new();
    for num in 2..=lim {
        while *N % num == 0 {
            *N /= num;
            fac_vec.push(num);
        }
    }
    if *N >= 2 {
        fac_vec.push(*N);
    }
    fac_vec
}

#[allow(dead_code)]
#[allow(non_snake_case)]
fn divisor_enum() {
    input! {
        N: u64,
    }
    print_divisor(N);
}

#[allow(dead_code)]
#[allow(non_snake_case)]
fn print_divisor(N: u64) {
    let lim: u64 = (N as f64).sqrt() as u64;
    for num in 1..=lim {
        if N % num != 0 { continue; }
        println!("{}", num);
        let other_div = N / num;
        if num != other_div {
            println!("{}", other_div);
        }
    }
}

/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_l
#[allow(non_snake_case)]
#[allow(dead_code)]
fn primality_test() {
    input! {
        N: u64,
    }
    println!("{}", yes_or_no(is_prime2(N)));

}

/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_k
#[allow(non_snake_case)]
#[fastout]
fn prime_number1() {
    input!{
        N:u16,
    }
    let mut vec: Vec<String> = Vec::new();
    for num in 2..=N {
        if is_prime(num) {
            vec.push(num.to_string());
        }
    }
    println!("{}", vec.join(" "));
}

fn is_prime2(num: u64) -> bool {
    let lim = (num as f64).sqrt() as u64;
    for i in 2..=lim {
        if num % i == 0 {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
fn is_prime(num: u16) -> bool {
    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }
    true
}




/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_j
#[allow(non_snake_case)]
#[fastout]
fn factorial() {
    input!{
        N: u64,
    }

    let mut ans = 1;
    for i in 2..=N {
        ans *= i
    }
    println!("{}", ans);
}

/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_j
#[allow(non_snake_case)]
#[fastout]
fn factorial2() {
    input! {
        N: u64,
    }
    println!("{}", fac(N));
}

#[allow(dead_code)]
#[allow(non_snake_case)]
fn fac(N: u64) -> u64 {
    match N {
        0 | 1 => 1,
        _ => N * fac(N - 1),
    }
}

#[allow(dead_code)]
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