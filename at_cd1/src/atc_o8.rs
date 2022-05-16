use proconio::{ input, fastout };

fn main() {
    friends();
}

#[allow(dead_code)]
fn friends() {
    input! {
        n: usize, m: usize,
        ab_vec: [(isize, isize); m],
    }

    let mut parent: Vec<isize> = vec![-1; n];
    for (a, b) in ab_vec {
        unite(&mut parent, a-1, b-1);
    }

    let mut cnt = 0;
    for i in 0..n {
        cnt = cnt.max(size_u(&parent, i as isize));
    }
    // println!("{:?}", parent);
    println!("{}", cnt);
}

fn size_u(parent: &Vec<isize>, x: isize) -> isize {
    -parent[root(parent, x) as usize]
}

fn unite(parent: &mut Vec<isize>, mut x: isize, mut y: isize) -> bool {
    x = root(parent, x);
    y = root(parent, y);
    if x == y {
        return false;
    }
    if parent[x as usize] > parent[y as usize] {
        std::mem::swap(&mut x, &mut y);
    }
    parent[x as usize] += parent[y as usize];
    parent[y as usize] = x;
    true
}

fn root(parent: &Vec<isize>, x: isize) -> isize {
    if parent[x as usize] < 0 {
        return x;
    }
    root(parent, parent[x as usize])
}

#[allow(dead_code)]
fn staircase_sequences() {
    input! {
        mut n: usize,
    }
    let mut i = 1;
    let mut cnt = 0;
    while i * i <= 2*n {
        if 2*n % i == 0 {
            let x = i;
            let y = 2*n / x;
            if x % 2 != y % 2 {
                cnt += 2;
            }
        }
        i += 1;
    }
    println!("{}",  cnt);
}

#[allow(dead_code)]
fn prediction_and_restriction() {
    input! {
        n: usize, k: usize,
        r: usize, s: usize, p: usize,
        t: String,
    }
    let mut t_vec: Vec<char> = t.chars().collect();
    for i in k..n {
        if t_vec[i-k] == t_vec[i] {
            t_vec[i] = 'x';
        }
    }

    let mut sum = 0;
    for i in 0..n {
        sum += match t_vec[i] {
            'r' => p,
            's' => r,
            'p' => s,
            _ => 0,
        }
    }
    println!("{}", sum);
}

#[allow(dead_code)]
fn i_hate_factorization() {
    input! {
        x: i64,
    }
    for a in -1000..=1000 {
        for b in -1000..=1000 {
            if (a as i64).pow(5) - (b as i64).pow(5) == x {
                println!("{} {}", a, b);
                return;
            }
        }
    }
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