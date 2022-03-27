use proconio::{ input, fastout };

fn main() {
    combination_hard();
}


/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ar
fn combination_hard() {
    println!("TODO");
}


/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_aq
#[allow(dead_code)]
#[allow(non_snake_case)]
#[fastout]
fn power() {
    input! {
        a: u64,
        b: u64,
    }
    println!("{}", mod_pow(a, b));

}

fn mod_pow(a: u64, b: u64) -> u64 {
    const MOD: u64 = 1000000007;
    let mut ans: u64 = 1;
    let mut pow: u64 = a;
    for i in 0..30 {
        if (b & (1 << i)) != 0 {
            ans *= pow;
            ans %= MOD;
        }
        pow *= pow;
        pow %= MOD;
    }
    ans
}


/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ap
#[allow(non_snake_case)]
#[allow(non_snake_case)]
#[fastout]
fn fibonacci_easy_mod() {
    input! {
        N: usize,
    }
    const MOD: usize = 1000000007;
    let mut a: Vec<usize> = vec![0; N+1];

    a[1] = 1;
    a[2] = 1;
    for n in 3..=N {
        a[n] = (a[n-1] + a[n-2]) % MOD;
        // a[n] = (a[n-1] % MOD) + (a[n-2] % MOD); 
    }

    println!("{}", a[N] % MOD);
    // println!("{}", a[N]);
}

