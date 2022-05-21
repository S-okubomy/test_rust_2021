use proconio::{ input, fastout };
use std::collections::{ HashSet, VecDeque };

fn main() {
    double_dots();
}

#[allow(dead_code)]
fn double_dots() {
    input! {
        n: usize, m: usize,
        ab_vec: [(usize, usize); m],
    }
    let mut connect: Vec<Vec<usize>> = vec![vec![]; n+1];
    for (a, b) in ab_vec {
        connect[a].push(b);
        connect[b].push(a);
    }
    // println!("{:?}", connect);
    let mut deque: VecDeque<usize> = VecDeque::new();
    deque.push_back(1);
    let mut visit_vec: Vec<bool> = vec![false; n+1];
    let mut ans_vec: Vec<usize> = vec![0; n+1];
    while deque.len() > 0 {
        let pos = deque.pop_front().unwrap();
        for c in &connect[pos] {
            if !visit_vec[*c] {
                deque.push_back(*c);
                visit_vec[*c] = true;
                ans_vec[*c] = pos; 
            }
        }
    }

    println!("Yes");
    for i in 2..=n {
        println!("{}", ans_vec[i]);
    }
}

#[allow(dead_code)]
fn teleporter2() {
    input! {
        n: usize, mut k: isize,
        mut a_vec: [isize; n],
    }
    a_vec.insert(0, 0);
    let mut visited_vec: Vec<isize> = vec![-1; n+1];
    visited_vec[1] = 0;
    let mut now_town = 1;
    let mut move_cnt = 0;
    let mut cycle = 0;
    for _i in 0..1_000_000 {
        move_cnt += 1;
        now_town = a_vec[now_town as usize];
        if move_cnt == k {
            println!("{}", now_town);
            return;
        }
        if visited_vec[now_town as usize] == -1 {
            visited_vec[now_town as usize] = move_cnt;
        } else {
            cycle = move_cnt - visited_vec[now_town as usize];
            break;
        }
    }
    k -= move_cnt;
    k %= cycle;
    for _i in 0..k {
        now_town = a_vec[now_town as usize];
    }
    println!("{}", now_town);
}

#[allow(dead_code)]
fn teleporter() {
    input! {
        n: usize, mut k: isize,
        mut a_vec: [isize; n],
    }
    a_vec.insert(0, 0);
    let mut visited_vec: Vec<isize> = vec![-1; n+1];
    visited_vec[1] = 0;
    let mut now_town = 1;
    let mut move_cnt: isize = 0;
    let mut cycle: isize = 0;
    for i in 0..=1_000_000 {
        move_cnt += 1;
        now_town = a_vec[now_town as usize];
        if move_cnt == k {
            println!("{}", now_town);
            return;
        }
        if visited_vec[now_town as usize] == -1 {
            visited_vec[now_town as usize] = move_cnt;
        } else {
            cycle = move_cnt - visited_vec[now_town as usize];
            break;
        }
    }
    k -= move_cnt;
    k %= cycle;
    for i in 0..k {
        now_town = a_vec[now_town as usize];
    }
    println!("{}", now_town);
}

#[allow(dead_code)]
fn buy_an_integer() {
    input! {
        a:u64, b: u64, x: u64,
    }
    let mut left: u64 = 0;
    let mut right: u64 = 1000_000_001;
    while 1 < right - left {
        let mid = (left + right) / 2;
        if check(a, b, mid) <= x {
            left = mid;
        } else {
            right = mid;
        }
    }
    println!("{}", left); 
}

fn check(a: u64, b: u64, x: u64) -> u64 {
    let dig_x: u64 = x.to_string().len() as u64;
    a * x + b * dig_x
}


#[allow(dead_code)]
fn tle_buy_an_integer() {
    input! {
        a:usize, b: usize, x: usize,
    }
    for i in 1..=1_000_000_000 {
        if a * i + b * get_dig(i.to_string()) > x {
            println!("{}", i-1);
            return;
        }
    }
}

fn get_dig(s: String) -> usize {
    s.len()
}

#[allow(dead_code)]
fn div_game() {
    input! {
        mut n: usize,
    }

    if n == 1 {
        println!("0");
        return;
    }
    // println!("{:?}", get_prime_vec(n));
    let prime_set: HashSet<usize> = get_prime_vec(n).into_iter().collect();
    let mut cnt = 0;
    for p in prime_set {
        for e in 1..=10_usize.pow(10) {
            if n % p.pow(e as u32) == 0{
                cnt += 1;
                n /= p.pow(e as u32);
            } else {
                break;
            }
        }
    }
    println!("{}", cnt);
}

fn get_prime_vec(mut n: usize) -> Vec<usize> {
    let mut prime_vec: Vec<usize> = Vec::new();
    if n == 1 {
        prime_vec.push(1);
        return prime_vec;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            prime_vec.push(i);
            n /= i;
            // println!("{} {}", n, i);
        } else {
            i += 1;
        }
    }
    if n != 1 {
        prime_vec.push(n);
    }
    prime_vec
}

#[allow(dead_code)]
fn wa_div_game() {
    input! {
        n: usize,
    }
    let mut prime_vec: Vec<(usize, usize)> = Vec::new();
    let lim: usize = (n as f64).sqrt() as usize;
    for i in 2..=lim {
        if n % i == 0 {
            let mut e = 0;
            let mut cur_n = n;
            while cur_n % i == 0 {
                cur_n /= i;
                e += 1;
            }
            prime_vec.push((i, e));
        }
    }
    let mut cnt = 0;
    for (p, e) in prime_vec {
        let mut res = 1;
        while (res + 1) * (res + 2) / 2 <= e {
            res += 1;
        }
        cnt += res;
    }
    if n !=1 {
        cnt += 1;
    }
    println!("{}", cnt);
}

fn union_find() {
    input! {
        n: usize, q: usize,
        pab_vec: [(usize, usize, usize); q],
    }
    let mut uni: UnionFind = UnionFind::new(n);
    for (p, a, b) in pab_vec {
        if p == 0 {
            uni.unite(a, b);
        } else {
            if uni.is_same(a, b) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent: Vec<usize> = vec![0; n];
        for i in 0..n {
            parent[i] = i; // 最初は全てが根であるとして初期化
        }
        Self { parent }
    }

    fn root(&self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }
        self.root(self.parent[x])
    }

    fn unite(&mut self, a: usize, b: usize) {
        let r_a = self.root(a);
        let r_b = self.root(b);
        if r_a == r_b {
            return;
        }
        self.parent[r_a] = r_b; // aの根r_aをbの根r_bにつける
    }

    fn is_same(&self, a: usize, b: usize) -> bool {
        let r_a = self.root(a);
        let r_b = self.root(b);
        r_a == r_b
    }
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