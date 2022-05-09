use proconio::{ input, fastout };

fn main() {
    ubiquity();
}

#[allow(dead_code)]
fn ubiquity() {
    input! {
        n: i64,
    }
    const MOD: i64 = 1_000_000_000 + 7;
    let p10: i64 = get_p_mod(10, n, MOD);
    let p9: i64 = get_p_mod(9, n, MOD);
    let p8: i64 = get_p_mod(8, n, MOD);
    // println!("{} {} {}", p10, p9, p8);
    let mut ans = p10 - p9 - p9 + p8;
    ans %= MOD;
    ans = (ans + MOD) % MOD; // TODO 理由確認
    println!("{}", ans);
}

fn get_p_mod(x: i64, n: i64, mod_p: i64) -> i64 {
    let mut res: i64 = 1;
    for _i in 1..=n {
        res = (res * x) % mod_p;
    }
    res % mod_p
}