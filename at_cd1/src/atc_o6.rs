use proconio::{ input, fastout };
use std::cmp::{ min, max };
use std::collections::{ HashSet, HashMap };
use itertools::Itertools;

fn main() {
    route_map();
}

#[allow(dead_code)]
fn route_map() {
    input! {
        n: usize, m: usize,
        s_vec: [String; n],
        t_vec: [String; m],
    }

    let stop_st_set: HashSet<String> = t_vec.into_iter().collect();
    for s in s_vec {
        if stop_st_set.contains(&s) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

#[allow(dead_code)]
fn who_is_missing() {
    input! {
        n: usize,
        a_vec: [usize; 4*n-1],
    }

    let mut a_map: HashMap<usize, usize> = HashMap::new();
    for a in a_vec {
        let cnt = a_map.entry(a).or_insert(0);
        * cnt += 1;
    }
    // println!("{:?}", a_map);
    for a in a_map {
        if a.1 == 3 {
            println!("{}", a.0);
            return;
        }
    }
}

#[allow(dead_code)]
fn chukodai() {
    input! {
        s: String,
        a: usize, b: usize,
    }
    let mut s_vec: Vec<char> = s.chars().collect();
    s_vec.swap(a-1, b-1);
    let ans: String = s_vec.iter().collect();
    println!("{}", ans);
}

#[allow(dead_code)]
fn kasaka() {
    input! {
        s: String,
    }
    let s_vec: Vec<char> = s.chars().collect();
    let mut a_rev_cnt = 0;
    let rev_string_vec: Vec<char> = s.chars().rev().collect();   // 文字列を逆順
    let mut a_cnt = 0;
    for s in s_vec {
        if 'a' == s {
            a_cnt +=1;
        } else {
            break;
        }
    } 
    
    for s_rev in rev_string_vec {
        if 'a' == s_rev {
            a_rev_cnt += 1;
        } else {
            break;
        }
    }
    if a_rev_cnt < a_cnt {
        println!("No");
        return;
    }
    let add_a_string: String = "a".repeat(a_rev_cnt-a_cnt) + &s;
    let rev_add_a_string: String = add_a_string.chars().rev().collect();
    // println!("{}", add_a_string);
    // println!("{}", rev_add_a_string);
    if add_a_string == rev_add_a_string {
        println!("Yes");
    } else {
        println!("No");
    }
}

#[allow(dead_code)]
fn kasaka_tle() {
    input! {
        s: String,
    }

    let s_vec: Vec<char> = s.chars().collect();
    let a_int_cnt: usize = s_vec.iter().filter(|c| **c == 'a').count();
    let mut a_cnt = 0;
    let mut add_string: String = s;
    let mut rev_string: String = add_string.chars().rev().collect();   // 文字列を逆順
    if a_int_cnt == 0 && add_string != rev_string {
        println!("No");
        return;
    }

    while a_cnt <= a_int_cnt && a_cnt < 10_usize.pow(6) {
        if add_string == rev_string {
            println!("Yes");
            return;
        }
        // 先頭に'a'を加える
        add_string = "a".to_string() + &add_string;
        rev_string = add_string.chars().rev().collect();
        a_cnt += 1;
    }
    println!("No");
}

#[allow(dead_code)]
fn matrix_transposition() {
    input! {
        h: usize, w: usize,
        a_vec: [[usize;w]; h],
    }
    for j in 0..w {
        for i in 0..h {
            print!("{} ", a_vec[i][j]);
        }
        println!("");
    }
}

#[allow(dead_code)]
fn not_overflow() {
    input! {
        n: i64,
    }

    if - (1<<31) <= n && n < (1<<31) {
        println!("Yes");
    } else {
        println!("No");
    }
}

#[allow(dead_code)]
fn knight_fork() {
    input! {
        x1: isize, y1: isize, x2: isize, y2: isize,
    }

    for x in (x1-2)..=(x1+2) {
        for y in (y1-2)..=(y1+2) {
            if dist(x, y, x1, y1) == 5 && dist(x, y, x2, y2) == 5 {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

#[allow(dead_code)]
fn dist(a: isize, b: isize, c: isize, d: isize) -> isize {
    (a - c).pow(2) + (b - d).pow(2) 
}

#[allow(dead_code)]
fn integer_division() {
    input! {
        x: isize,
    }

    let ans: isize;
    if x >= 0 {
        ans = x / 10;
    } else {
        if x % 10 == 0 {
            ans = x / 10;
        } else {
            ans = x / 10 -1;
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn horizon() {
    input! {
        h: f64,
    }
    let ans: f64 = (h * (12_800_000_f64 + h)).sqrt();
    println!("{}", ans);
}

#[allow(dead_code)]
fn jumping_takahashi() {
    input! {
        n: usize, x: usize,
        mut ab_vec: [(usize, usize); n],
    }
    ab_vec.insert(0, (0, 0));

    // i回ジャンプして座標jにいるかどうか
    let mut dp: Vec<Vec<bool>> = vec![vec![false; x+1]; n+1];
    dp[0][0] = true;
    for i in 1..=n {
        let (a, b) = ab_vec[i];
        for j in 1..=x {
            if j >= a && dp[i-1][j-a] {
                dp[i][j] = true;
            } else if j >= b && dp[i-1][j-b] {
                dp[i][j] = true;
            }
        }
    }
    // println!("{:?}", dp);
    println!("{}", yes_or_no(dp[n][x]));
}

#[allow(dead_code)]
fn count_distinct_integers() {
    input! {
        n: usize,
        a_vec: [usize; n],
    }
    let a_map: HashSet<usize> = a_vec.into_iter().collect();
    println!("{}", a_map.len());
}

#[allow(dead_code)]
fn edge_checker() {
    input! {
        a: u8, b: u8,
    }
    if ((a % 10) + 1 == b) || (b % 10) + 1 == a {
        println!("Yes");
    } else {
        println!("No");
    }
}

#[allow(dead_code)]
fn connect6() {
    input! {
        n: usize,
        s: [String; n],
    }
    let map: Vec<Vec<char>> = s.iter().map(|s| { s.chars().collect::<Vec<char>>() }).collect::<Vec<Vec<char>>>();
    let dir_vec: Vec<(isize, isize)> = vec![(0, 1), (1, 0), (1, 1), (1, -1)]; // 縦方向、横方向、右上方向、右下方向
    for i in 0..n {
        for j in 0..n {
            let posi = (i as isize, j as isize);
            for dir in &dir_vec {
                if can_paint(n as isize, posi, *dir, &map) {
                    println!("Yes");
                    return;
                }    
            }
        }
    }
    println!("No");
}

#[allow(dead_code)]
fn can_paint(n: isize, posi: (isize, isize), dir: (isize, isize), map: &Vec<Vec<char>>) -> bool {
    let (mut x, mut y) = posi;
    let (dx, dy) = dir;
    let mut cnt = 0;

    for _ in 1..=6 {
        if !((0 <= x && x < n) && (0 <= y && y < n)) {
        // if !(0 <= min(x, y) && max(x,y) < n) {
             return false;
        }
        match map[x as usize][y as usize] {
            '#' => {
                cnt += 1;
            },
            _ => (),
        }
        x += dx;
        y += dy;
    } 
    // while (serch_cnt < 6) && (0 <= x && x < n) && (0 <= y && y < n) {
    // while (serch_cnt < 6) && 0 <= min(x, y) && max(x,y) < n {
    //     match map[x as usize][y as usize] {
    //         '#' => {
    //             cnt += 1;
    //         },
    //         _ => (),
    //     }
    //     x += dx;
    //     y += dy;
    //     serch_cnt += 1;
    // }  

    return cnt >= 4; 
} 

#[allow(dead_code)]
fn pasta() {
    input! {
        n: usize, m: usize,
        a_vec: [usize; n],
        b_vec: [usize; m],
    }
    let mut pasta_map: HashMap<usize, usize> = HashMap::new();
    for a in a_vec {
        let cnt = pasta_map.entry(a).or_insert(0);
        *cnt += 1;
    }
    for b in b_vec {
        if pasta_map.contains_key(&b) {
            let cnt: usize = *pasta_map.get(&b).unwrap();
            if cnt > 0 {
                let cnt = pasta_map.entry(b).or_default();
                *cnt -= 1;
            } else {
                println!("No");
                return;
            }
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

#[allow(dead_code)]
fn digit_machine() {
    input! {
        a_vec: [u8; 10],
    }

    let mut k = 0;
    for _i in 1..=2 {
        k = a_vec[k as usize];
    }
    println!("{}", a_vec[k as usize]);
}



#[allow(dead_code)]
fn gal_password() {
    input! {
        n: usize,
    }

    const MOD: usize = 998244353;
    // 上からi桁目までを決めて、jとなる組み合わせ数（を998244353で割った余り）
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 11]; n+1];
    dp[0][0] = 0;
    for j in 1..=9 {
        dp[1][j] = 1;
    }

    for i in 2..=n {
        for j in 1..=9 {
            dp[i][j] = (dp[i-1][j-1] + dp[i-1][j] + dp[i-1][j+1]) % MOD;
        }
    }
    let ans: usize = dp[n].iter().sum(); 
    println!("{}", ans % MOD);
    // println!("{:?}", dp);
}

#[allow(dead_code)]
fn minimize_ordering() {
    input! {
        s: String,
    }
    let mut s_vec: Vec<char> = s.chars().collect();
    s_vec.sort_by(|a, b| a.cmp(b));
    let join_string: String = s_vec.iter().map(|c| c.to_string()).collect();
    println!("{}", join_string);
}

#[allow(dead_code)]
fn tle_minimize_ordering() {
    input! {
        s: String,
    }
    let s_vec: Vec<String> = s.chars().map(|c| c.to_string()).collect();
    let mut join_vec: Vec<String> = Vec::new();
    let s_len = s.len();
    for perm in s_vec.iter().permutations(s_len) {
        let s_perm: String = perm.iter().map(|s| s.to_string()).collect();
        join_vec.push(s_perm);
    }
    join_vec.sort_by(|a, b| a.cmp(b));
    println!("{}", join_vec[0]);
}

#[allow(dead_code)]
fn t_shirt() {
    input! {
        a: f64, b: f64, c: f64, x: f64,
    }

    if x <= a {
        println!("{}", 1);
    } else if a < x && x <= b {
        let prob: f64 = c / (b - a);
        println!("{}", prob);
    } else {
        println!("{}", 0);
    }
}

#[allow(dead_code)]
fn collision2() {
    input! {
        n: usize,
        xy_vec : [(usize, usize); n],
        s: String,
    }
    let y_map: HashSet<usize> = xy_vec.iter().map(|xy| xy.1).collect();
    let s_vec: Vec<String> = s.chars().map(|c| c.to_string()).collect(); 
    let mut dir_r_map: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut dir_l_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..n {
        let (x, y) = xy_vec[i];
        match s_vec[i].as_str() {
            "R" => {
                dir_r_map.entry(y).or_insert(vec![]).push(x);
            },
            "L" => {
                dir_l_map.entry(y).or_insert(vec![]).push(x);
            },
            _ => (),
        }
    }

    for y in y_map {
        if dir_l_map.contains_key(&y) && dir_r_map.contains_key(&y) {
            let dir_l_max: usize = *dir_l_map.get(&y).unwrap().iter().max_by(|a, b| a.cmp(b)).unwrap();
            let dir_r_min: usize = *dir_r_map.get(&y).unwrap().iter().min_by(|a, b| a.cmp(b)).unwrap();
            if dir_r_min <= dir_l_max {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
} 

#[allow(dead_code)]
fn hit_and_blow() {
    input! {
        n: usize,
        a_vec: [usize; n],
        b_vec: [usize; n],
    }
    // A にも B にも含まれ、その位置も一致している整数の個数
    let mut same_cnt = 0;
    for i in 0..n {
        if a_vec[i] == b_vec[i] {
            same_cnt += 1;
        }
    }
    println!("{}", same_cnt);

    // A にも B にも含まれるが、その位置は異なる整数の個数。
    let mut diff_cnt = 0;
    for i in 0..n {
        for j in 0..n {
            if (i != j) && (a_vec[i] == b_vec[j]) {
                diff_cnt += 1;
            }
        }
    }
    println!("{}", diff_cnt);
}

#[allow(dead_code)]
fn shampoo() {
    input! {
        v: usize, a: usize, b: usize, c: usize,
    }
    let remain: usize = v % (a + b + c);
    if remain < a {
        println!("F");
    } else if remain < a + b {
        println!("M");
    } else if remain < a + b + c {
        println!("T");
    }
}

#[allow(dead_code)]
fn go_straight_and_turn_right() {
    input! {
        _n: usize,
        t: String,
    }
    let t_vec: Vec<String> = t.chars().map(|c| c.to_string()).collect();
    let dir_vec: Vec<(isize, isize)> = vec![(1, 0), (0,-1), (-1, 0), (0, 1)]; //東、 南、西、北
    let mut dir_no = 0;
    let mut posi: (isize, isize) = (0, 0); 
    for t in t_vec {
        match t.as_str() {
            "R" => {
                dir_no = (dir_no + 1) % 4;
            },
            "S" => {
                posi.0 += dir_vec[dir_no].0;
                posi.1 += dir_vec[dir_no].1;
            },
            _=> (),
        }
    }
    println!("{} {}", posi.0, posi.1);
}

#[allow(dead_code)]
fn last_letter() {
    input! {
        n: usize,
        s: String,
    }
    println!("{}", &s[n-1..n]);
}

#[allow(dead_code)]
fn choose_elements2() {
    input! {
        n: usize, k: isize,
        mut a_vec: [isize; n],
        mut b_vec: [isize; n],
    }
    a_vec.insert(0, 0);
    b_vec.insert(0, 0);

    let mut dp: Vec<Vec<bool>> = vec![vec![false; n+1]; 3];
    dp[1][1] = true;
    dp[2][1] = true;
    for j in 2..=n {
        // A[j-1]からの繋がり確認
        if dp[1][j-1] {
            if (a_vec[j-1] - a_vec[j]).abs() <= k {
                dp[1][j] = true;
            }
            if (a_vec[j-1] - b_vec[j]).abs() <= k {
                dp[2][j] = true;
            }
        }
        // B[j-1]からの繋がり確認
        if dp[2][j-1] {
            if (b_vec[j-1] - a_vec[j]).abs() <= k {
                dp[1][j] = true;
            }
            if (b_vec[j-1] - b_vec[j]).abs() <= k {
                dp[2][j] = true;
            }
        }
    }
    if dp[1][n] || dp[2][n] {
        println!("Yes");
    } else {
        println!("No");
    }
}

#[allow(dead_code)]
fn choose_elements() {
    input! {
        n: usize, k: isize,
        mut a_vec: [isize; n],
        mut b_vec: [isize; n],
    }
    a_vec.insert(0, 0);
    b_vec.insert(0, 0);

    // A又はBを選択して条件を満たすかどうか
    let mut dp: Vec<Vec<bool>> = vec![vec![false; n+1]; 3];
    dp[1][1] = true;
    dp[2][1] = true;
    for j in 2..=n {
        // A側の確認
        if dp[1][j-1] {
            if (a_vec[j-1] - a_vec[j]).abs() <= k {
                dp[1][j] = true;
            }
            if (a_vec[j-1] - b_vec[j]).abs() <= k {
                dp[2][j] = true;
            }
        }
        // B側の確認
        if dp[2][j-1] {
            if (b_vec[j-1] - a_vec[j]).abs() <= k {
                dp[1][j] = true;
            }
            if (b_vec[j-1] - b_vec[j]).abs() <= k {
                dp[2][j] = true;
            }
        }
    }

    // println!("{:?}", dp);

    if dp[1][n] || dp[2][n] {
        println!("Yes");
    } else {
        println!("No");
    }
}

#[allow(dead_code)]
fn mex() {
    input! {
        n: usize,
        a_vec: [usize; n],
    }

    for x in 0..=2000 {
        if !a_vec.iter().any(|a| *a == x) {
            println!("{}", x);
            return;
        }
    }
} 

#[allow(dead_code)]
fn good_morning() {
    input! {
        a: u8, b: u8, c: u8, d: u8,
    }
    if a < c {
        println!("Takahashi");
    } else if a == c && b <= d {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}

#[allow(dead_code)]
fn greatest_common_divisor() {
    input! {
        a: usize, b: usize,
    }
    println!("{}", gcd(a, b));
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

#[allow(dead_code)]
fn factorization() {
    input! {
        n: usize,
    }
    let mut pri = n;
    let lim: usize = (n as f64).sqrt() as usize;
    for x in 2..=lim {
        while pri % x == 0 {
            print!("{} ", x);
            pri /= x;
        }
    }
    if pri >= 2 {
        print!("{}", pri);
    }
}

#[allow(dead_code)]
fn divisor_enumeration() {
    input! {
        n: usize,
    }
    let mut ans_vec: Vec<usize> = Vec::new();
    let lim: usize = (n as f64).sqrt() as usize;
    for x in 1..=lim {
        if n % x == 0 {
            ans_vec.push(x);
            ans_vec.push(n / x);
        }
    }
    ans_vec.sort_by(|a, b| a.cmp(b));
    for ans in ans_vec {
        println!("{}", ans);
    }
}

#[allow(dead_code)]
fn primality_test() {
    input! {
        n: usize,
    }

    let lim: usize = (n as f64).sqrt() as usize;
    for i in 2..=lim {
        if n % i == 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

#[allow(dead_code)]
fn print_prime_numbers() {
    input! {
        n: usize,
    }

    for x in 2..=n {
        if is_prime(x) {
            print!("{} ", x)
        }
    } 
    println!("");
}

#[allow(dead_code)]
fn is_prime(x: usize) -> bool {
    let lim: usize = ((x as f64).sqrt()) as usize;  
    for i in 2..=lim {
        if x % i == 0 {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
fn factorial() {
    input! {
        n: usize,
    }
    println!("{}", fact(n));
}

#[allow(dead_code)]
fn fact(x: usize) -> usize {
    match x {
        0 | 1 => 1,
        _ => x * fact(x-1),
    }
}

#[allow(dead_code)]
fn brute_force2() {
    input! {
        n: usize, s: usize,
        mut a_vec: [usize; n],
    }
    a_vec.insert(0, 0);
    // i番目のカード選んだ時、合計がjとなるかどうか
    let mut dp: Vec<Vec<bool>> = vec![vec![false; s+1]; n+1];
    dp[0][0] = true;
    for i in 1..=n {
        let card_no = a_vec[i];
        for j in 0..=s {
            if j >= card_no {
                // if dp[i-1][j] || (dp[i-1][j-card_no]) {
                //     dp[i][j] = true;
                // }
                dp[i][j] = dp[i-1][j] | dp[i-1][j-card_no];
            } else {
                dp[i][j] = dp[i-1][j];
            }
        }
    }
    println!("{}", yes_or_no(dp[n][s]));
    // println!("{:7?}", dp);
    /*
        3 11
        2 5 9
            合計    0         1       2        3       4         5        6        7        8        9        10       11
            0番目 [true   , false  , false  , false  , false  , false  , false  , false  , false  , false  , false  , false  ]
            1番目 [true  , false  , true   , false  , false  , false  , false  , false  , false  , false  , false  , false  ] 
            2番目 [true  , false  , true   , false  , false  , true   , false  , true   , false  , false  , false  , false  ]
            3番目 [true  , false  , true   , false  , false  , true   , false  , true   , false  , true   , false  , true   ]
    */
}

#[allow(dead_code)]
fn yes_or_no(b: bool) -> String {
    if b {
        String::from("Yes")
    } else {
        String::from("No")
    }
}


#[allow(dead_code)]
fn brute_force1() {
    input! {
        n: usize, s: usize,
    }

    let mut cnt = 0;
    for i in 1..=n {
        for j in 1..=n {
            if i + j <= s {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}


#[allow(dead_code)]
fn number_of_multiples1() {
    input! {
        n: usize, x: usize, y: usize,
    }

    let mut cnt = 0;
    for i in 1..=n {
        if (i % x == 0) || (i % y == 0) {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}

#[allow(dead_code)]
fn print_2() {
    input! {
        n: usize,
    }
    println!("{}", 2 * n + 3);
}

#[allow(dead_code)]
fn modulo_100() {
    input! {
        n: usize,
        a_vec: [usize; n],
    }
    let ans: usize = a_vec.iter().fold(0, |acc, cur| (acc + cur) % 100);
    println!("{}", ans);
}

#[allow(dead_code)]
fn product_of_3_integers() {
    input! {
        a_vec: [usize; 3],
    }
    let ans: usize = a_vec.iter().fold(1, |acc, cur| acc * cur);
    println!("{}", ans); 
}

#[allow(dead_code)]
fn sum_of_n_integers() {
    input! {
        n: usize,
        a_vec: [usize; n],
    }

    println!("{}", a_vec.iter().sum::<usize>());
}

#[allow(dead_code)]
fn sum_of_3_integers() {
    input! {
        a1: usize, a2: usize, a3: usize,
    }
    println!("{}", a1+a2+a3);
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn print_5n() {
    input! {
        n: u8,
    }
    println!("{}", n+5);
}