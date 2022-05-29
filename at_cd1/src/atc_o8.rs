use proconio::{ input, fastout };
use std::collections::{ HashSet, VecDeque, BinaryHeap };
use std::f64::consts::PI;
use std::cmp::{ min, max };
use num::{ Zero, One };
use std::ops::{ MulAssign, RemAssign };

fn main() {
    crested_ibis_vs_monster2();
}

#[allow(dead_code)]
fn crested_ibis_vs_monster2() {
    input! {
        h: usize, n: usize,
        mut ab_vec: [(usize, usize); n],
    }
    // dp[j]: ダメージ jを与える最小の魔法消費量．dp[h]のみダメージ量がh以上
    let mut dp: Vec<usize> = vec![10_000_000_000; h+1];
    dp[0] = 0;
    for i in 0..n {
        let (a, b) = ab_vec[i];
        for j in 0..=h {
            // j+a <= h の時は、dp[j+a] に設定
            // j+a > hの時は、dp[h] に設定
            dp[min(j+a, h)] = min(dp[min(j+a, h)], dp[j] +b);
        }
    }
    println!("{}", dp[h]);
}

#[allow(dead_code)]
fn crested_ibis_vs_monster() {
    input! {
        h: usize, n: usize,
        mut ab_vec: [(usize, usize); n],
    }
    // dp[j]: ダメージ jを与える最小の魔法消費量．dp[h]のみダメージ量がh以上
    let mut dp: Vec<usize> = vec![10_000_000_000; h+1];
    dp[0] = 0;
    for i in 0..n {
        let (a, b) = ab_vec[i];
        for j in 0..=h {
            if j + a <= h {
                dp[j+a] = min(dp[j+a], dp[j] + b);
            } else {
                dp[h] = min(dp[h], dp[j] + b);
            }
        }
    }
    println!("{}", dp[h]);
}

#[allow(dead_code)]
fn bouquet() {
    input! {
        n: usize, a: usize, b: usize,
    }
    const MOD: usize = 1_000_000_007;
    let mut ans = pow_mod(2, n, MOD) -1;
    ans = (ans + MOD - n_c_r(n, a, MOD)) % MOD;
    ans = (ans + MOD - n_c_r(n, b, MOD)) % MOD;
    println!("{}", ans);
}

fn n_c_r(n: usize, r: usize, _mod: usize) -> usize {
    let mut x = 1;
    let mut y = 1;
    for i in 1..=r {
        x *= n - i + 1; x %= _mod;
        y *= i; y %= _mod;
    }
    // nCr = x/y → x * 「分母の逆元」
    // 逆元計算方法
    // https://qiita.com/drken/items/6b4031ccbb2cab7436f3#4-1-%E9%80%86%E5%85%83%E3%82%92%E8%A8%88%E7%AE%97%E3%81%99%E3%82%8B
    x * pow_mod(y, _mod-2, _mod) % _mod
} 

// 繰り返し２乗法
fn pow_mod <T> (mut x:T, mut n:usize, _mod: T) -> T  //x^n
    where T: Copy + Zero + One + MulAssign + RemAssign,
{
    let mut res = T::one();
    while n > 0 {
        if n & 1 == 1 { // nを2進数で表した時の2^iの位が1の時に限り、x^2iが積に含まれる
            res *= x; res %= _mod;
        }
        x *= x; //一周する度にx, x^2, x^4, x^8となる
        x %= _mod;
        n /= 2; //桁をずらす n = n >> 1
    }
    res
}


#[allow(dead_code)]
fn maze_master() {
    input! {
        h: isize, w: isize,
        s: [String; h],
    }

    let map: Vec<Vec<char>> = s.iter().map(|s| s.chars().collect()).collect();
    let mut cnt = 0;
    for row in 0..h {
        for col in 0..w {
            if map[row as usize][col as usize] == '.' {
                cnt = max(cnt, get_bfs_max_cnt(w, h, row, col, &map));
            }
        }
    }
    println!("{}", cnt);
}

fn get_bfs_max_cnt(w: isize, h: isize, start_row: isize, start_col: isize, map: &Vec<Vec<char>>) -> isize {
    let dxy = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut map_cnt = vec![vec![-1; w as usize]; h as usize];
    map_cnt[start_row as usize][start_col as usize] = 0;
    let mut deque: VecDeque<(isize, isize)> = VecDeque::new();
    deque.push_back((start_row, start_col));
    while deque.len() > 0 {
        let (now_row, now_col) = deque.pop_front().unwrap();
        // println!("{} {}", now_x, now_y);
        let now_cnt = map_cnt[now_row as usize][now_col as usize];
        for (dx, dy) in &dxy {
            let next_row = now_row + dy;
            let next_col = now_col + dx;
            if (0 <= next_col && next_col < w) && (0 <= next_row && next_row < h) {
                if map[next_row as usize][next_col as usize] == '.' && map_cnt[next_row as usize][next_col as usize] == -1 {
                    map_cnt[next_row as usize][next_col as usize] = now_cnt + 1;
                    deque.push_back((next_row, next_col));
                }
            }
        }
        
    }
    let mut max_cnt = 0;
    for row in 0..h {
        for col in 0..w {
            max_cnt = max(max_cnt, map_cnt[row as usize][col as usize]);
        }
    }
    max_cnt
}

fn un_get_bfs_max_cnt(w: isize, h: isize, start_row: isize, start_col: isize, map: &Vec<Vec<char>>) -> isize {
    let dxy = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut map_cnt = vec![vec![-1; w as usize]; h as usize];
    map_cnt[start_row as usize][start_col as usize] = 0;
    let mut deque: VecDeque<(isize, isize)> = VecDeque::new();
    deque.push_back((start_col, start_row));
    while deque.len() > 0 {
        let (now_x, now_y) = deque.pop_front().unwrap();
        // println!("{} {}", now_x, now_y);
        let now_cnt = map_cnt[now_y as usize][now_x as usize];
        for (dx, dy) in &dxy {
            let x = now_x + dx;
            let y = now_y + dy;
            if (0 <= x && x < w) && (0 <= y && y < h) {
                println!("{} {}", y, h);
                if map[y as usize][x as usize] == '.' && map_cnt[y as usize][x as usize] == -1 {
                    map_cnt[y as usize][x as usize] = now_cnt + 1;
                    deque.push_back((x, y));
                }
            }
        }
        
    }
    let mut max_cnt = 0;
    for row in 0..h {
        for col in 0..w {
            max_cnt = max(max_cnt, map_cnt[row as usize][col as usize]);
        }
    }
    max_cnt
}

#[allow(dead_code)]
fn repsept() {
    input! {
        k: usize,
    }

    let mut amari = 0;
    for i in 1..=k {
        amari = amari * 10 + 7;
        if amari % k == 0 {
            println!("{}", i);
            return;
        }
        amari %= k;
    }
    println!("-1");
}

#[allow(dead_code)]
fn k_napsack1() {
    input! {
        n: usize, w: usize,
        mut wv_vec: [(usize, usize); n],
    }
    wv_vec.insert(0, (0, 0));

    // dp[i][wg]: iまでの品物を使って重さwgまでの荷物を入れた時の最大価値
    let mut dp: Vec<Vec<usize>> = vec![vec![0; w+1]; n+1];
    dp[0][0] = 0;
    for i in 1..=n {
        let (weight, val) = wv_vec[i];
        for wg in 0..=w {
            if wg >= weight {
                dp[i][wg] = max(dp[i-1][wg], dp[i-1][wg-weight] + val); 
            } else {
                dp[i][wg] = dp[i-1][wg];
            }
        }
    }
    // println!("{:?}", dp);
    println!("{}", dp[n][w]);
}

#[allow(dead_code)]
fn redistribution() {
    input! {
        s: usize,
    }
    const MOD: usize = 1000_000_007;
    let mut dp: Vec<usize> = vec![0; s+1];
    let mut sum = 0;
    for i in 3..=s {
        sum += dp[i-3];
        dp[i] = (sum + 1) % MOD;
    }
    // println!("{:?}", dp);        
    println!("{}", dp[s]);
}

#[allow(dead_code)]
fn opposite() {
    input! {
        n: f64,
        mut x0: f64, mut y0: f64,
        xn2: f64, yn2: f64,
    }

    let (c_x, c_y) = ((x0+xn2)/2_f64, (y0+yn2)/2_f64);

    x0 -= c_x;
    y0 -= c_y;

    let mut x1 = x0*(2_f64*PI/n).cos() - y0*(2_f64*PI/n).sin();
    let mut y1 = x0*(2_f64*PI/n).sin() + y0*(2_f64*PI/n).cos();
    x1 += c_x;
    y1 += c_y;
    println!("{} {}", x1, y1);
}

#[allow(dead_code)]
fn p_discount_tickets() {
    input!{
        n: usize, m: usize,
        a_vec: [usize; n],
    }

    let mut q: BinaryHeap<usize> = a_vec.into_iter().collect();
    for i in 0..m {
        let mut x: usize = q.pop().unwrap(); // get max val
        x /= 2;
        q.push(x);
    }
    let ans: usize = q.iter().sum(); 
    println!("{}", ans);
}

#[allow(dead_code)]
fn orxor() {
    input! {
        n: usize,
        a_vec: [usize; n],
    }
    // println!("{:08b}", 7);
    let mut xor_val: usize = 2_usize.pow(40);
    for bit in 0..1<<n+1 {
        if bit & 1 == 0 || bit>>n & 1 == 0 {
            continue;
        } 
        let mut parts_vec: Vec<usize> = Vec::new();
        for i in 0..n+1 {
            if (bit & 1<<i) != 0 {
                parts_vec.push(i);
            } 
        }
        // println!("{:?}", parts_vec);
        let mut tmp_xor_val = 0;
        for j in 0..parts_vec.len()-1 {
            tmp_xor_val = tmp_xor_val^calc_or(&a_vec, parts_vec[j], parts_vec[j+1]);
        }
        xor_val = xor_val.min(tmp_xor_val);
    }
    println!("{}", xor_val);
}

fn calc_or(a_vec: &Vec<usize>, left: usize, right: usize) -> usize {
    let mut or_val = 0;
    for i in left..right {
        or_val |= a_vec[i];
    }
    or_val
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