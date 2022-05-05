use proconio::{ input, fastout };
use std::cmp::{ min, max };
use std::collections::{ HashSet, HashMap };
use itertools::Itertools;

fn main() {
    triple_metre();
}

#[allow(dead_code)]
fn triple_metre() {
    input! {
        s: String,
    }
    let t: String = "oxx".repeat(10_usize.pow(5));
    println!("{}", if t.contains(&s) { "Yes" } else { "No" });
}

#[allow(dead_code)]
fn atcoder_quiz3() {
    input!{
        n:usize,
    }
    let ans;
    if n >= 42 {
        ans = n + 1;
    } else {
        ans = n;
    }

    println!("AGC{:0>3}", ans);
}

#[allow(dead_code)]
fn counting2() {
    input! {
        n: usize, q: usize,
        mut a_vec: [usize; n],
        x_vec: [usize; q],
    }
    a_vec.sort_by(|a, b| a.cmp(b));
    for x in x_vec {
        if x <= a_vec[0] {
            println!("{}", n);
            continue;
        }
        if a_vec[n-1] < x {
            println!("0");
            continue;
        }
        let mut l = 0;
        let mut r = n-1;
        while (r - l) > 1 {
            let mid = (r + l) / 2;
            if x <= a_vec[mid] {
                r = mid;
            } else {
                l = mid;
            }
        }
        println!("{}", n-r);
    }
}

#[allow(dead_code)]
fn election() {
    input! {
        n: usize,
        s_vec: [String; n],
    }
    let mut s_map: HashMap<String, usize> = HashMap::new();
    for s in s_vec {
        let cnt = s_map.entry(s).or_insert(0);
        *cnt += 1;
    }
    let max_name: String = s_map.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0.to_string();
    println!("{}", max_name);
}

#[allow(dead_code)]
fn water_pressure() {
    input! {
        d: f64,
    }
    let ans = d / 100_f64;
    println!("{}", ans);
}

#[allow(dead_code)]
fn product6() {
    input! {
        n: usize, x: usize,
        a_vec: [[usize]; n],
    }
    let ans = dfs6(0, 1, n, x, &a_vec);
    println!("{}", ans);
}

#[allow(dead_code)]
fn dfs6(posi: usize, seki: usize, n: usize, x: usize, a_vec: &Vec<Vec<usize>>) -> usize {
    let mut cnt = 0;
    if posi == n {
        if seki == x {
            cnt += 1;
        }
        return cnt;
    }

    for ai in &a_vec[posi] {
        if *ai > x / seki { continue; }
        cnt += dfs6(posi + 1, seki * ai, n, x, &a_vec);
    }
    cnt
}


#[allow(dead_code)]
fn product5() {
    input! {
        n: usize, x: usize,
        a_vec: [[usize]; n],
    }
    let mut seki_vec: Vec<usize> = Vec::new();
    seki_vec.push(1);
    for i in 0..n {
        let mut tmp_seki_vec: Vec<usize> = Vec::new();
        for seki in &seki_vec {
            for ai in &a_vec[i] {
                if *ai > x / *seki { continue; }
                tmp_seki_vec.push(seki * ai);
            }
        }
        seki_vec = tmp_seki_vec;
    }

    let ans = seki_vec.iter().filter(|seki| **seki == x).count();
    println!("{}", ans);
}

#[allow(dead_code)]
fn graph_isomorphism2() {
    input! {
        n: usize, m: usize,
        ab_vec: [(usize, usize); m],
        cd_vec: [(usize, usize); m],
    }
    let mut nb_ab: Vec<Vec<usize>> = vec![vec![]; n+1];
    for (a, b) in ab_vec {
        nb_ab[a].push(b);
        nb_ab[b].push(a);
    }
    nb_ab = nb_ab.into_iter().map(|mut ab| { ab.sort_by(|a, b| a.cmp(b)); ab }).collect();

    let perm_vec: Vec<usize> = (1..=n).collect();
    for perm in perm_vec.iter().permutations(n) {
        let mut nb_cd: Vec<Vec<usize>> = vec![vec![]; n+1];
        for (c, d) in &cd_vec {
            let c_cnv = *perm[*c-1];
            let d_cnv = *perm[*d-1];
            nb_cd[c_cnv].push(d_cnv);
            nb_cd[d_cnv].push(c_cnv);
        }
        nb_cd = nb_cd.into_iter().map(|mut cd| { cd.sort_by(|a, b| a.cmp(b)); cd }).collect();
        if nb_ab == nb_cd {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

#[allow(dead_code)]
fn graph_isomorphism() {
    input! {
        n: usize, m: usize,
        ab_vec: [(usize, usize); m],
        cd_vec: [(usize, usize); m],
    }
    let mut nb_ab: Vec<Vec<usize>> = vec![vec![]; n+1];
    for ab in ab_vec {
        let (a, b) = ab;
        nb_ab[a].push(b);
        nb_ab[b].push(a);
    }

    nb_ab = nb_ab.into_iter().map(|mut ab| { ab.sort_by(|a, b| a.cmp(b)); ab }).collect();

    let perm_vec: Vec<usize> = (1..=n).collect();
    for perm in perm_vec.iter().permutations(n) {
        println!("{:?}", perm);
        let mut nb_cd: Vec<Vec<usize>> = vec![vec![]; n+1];
        for (c, d) in &cd_vec {
            let c_cnv: usize = *perm[*c-1];
            let d_cnv = *perm[*d-1];
            nb_cd[c_cnv].push(d_cnv);
            nb_cd[d_cnv].push(c_cnv);
        }
        nb_cd = nb_cd.into_iter().map(|mut cd| { cd.sort_by(|a, b| a.cmp(b)); cd }).collect();
        println!("{:?}", nb_ab);
        println!("{:?}", nb_cd);
        if nb_ab == nb_cd {
            println!("Yes");
            return;
        } 
    }
    println!("No");

    /*
        4 4
        1 2
        1 3
        1 4
        3 4
        1 3
        1 4
        2 3
        3 4
        [[], [2, 3, 4], [1], [1, 4], [1, 3]]

        [1, 2, 3, 4]
        [[], [3, 4], [3], [1, 2, 4], [1, 3]]
        [1, 2, 4, 3]
        [[], [3, 4], [4], [1, 4], [1, 2, 3]]
        [1, 3, 2, 4]
        [[], [2, 4], [1, 3, 4], [2], [1, 2]]
        [1, 3, 4, 2]
        [[], [2, 4], [1, 4], [4], [1, 2, 3]]
        [1, 4, 2, 3]
        [[], [2, 3], [1, 3, 4], [1, 2], [2]]
    */
}

#[allow(dead_code)]
fn caesar_cipher() {
    input! {
        s: String,
        t: String,
    }

    // let sp = 'a' as u8;
    // println!("{}", sp);
    let s_vec: Vec<char> = s.chars().collect();
    for i in 0..26 {
        let tmp_s_vec: Vec<char> = s_vec.iter().map(|c| (97 + (*c as u8 - 97 + i) % 26) as char).collect();
        let tmp_string: String = tmp_s_vec.iter().map(|c| c.to_string()).collect();
        if tmp_string == t {
            println!("Yes");
            return;
        }
        // println!("{:?}", tmp_s_vec);
    }
    println!("No");
}

#[allow(dead_code)]
fn qq_solver() {
    input! {
        s: String,
    }
    let s_vec: Vec<usize> = s.chars().map(|c| c.to_string().parse::<usize>().unwrap_or(99)).collect();
    // println!("{:?}", s_vec);
    println!("{}", s_vec[0] * s_vec[2]);
}

#[allow(dead_code)]
fn product4() {
    input! {
        n: usize, x: usize,
        a_vec: [[usize]; n],
    }

    let ans = dfs2(0, 1, n, x, &a_vec);
    println!("{}", ans);
}

fn dfs2(posi: usize, seki: usize, n: usize, x: usize, a_vec: &Vec<Vec<usize>>) -> usize {
    let mut cnt = 0;
    if posi == n {
        if x == seki {
            cnt += 1;
        }
        return cnt;
    }

    for ai in &a_vec[posi] {
        if *ai > x / seki { continue; }
        cnt += dfs2(posi + 1, seki * ai, n, x, a_vec);
    }
    cnt
}


#[allow(dead_code)]
fn product3() {
    input! {
        n: usize, x: usize,
        a_vec: [[usize]; n],
    }

    let mut seki_vec: Vec<usize> = Vec::new();
    seki_vec.push(1);
    for i in 0..n {
        let mut tmp_seki_vec: Vec<usize> = Vec::new();
        for seki in &seki_vec {
            for ai in &a_vec[i] {
                if *ai > x / *seki { continue; }
                tmp_seki_vec.push(seki * ai);
            }
        }
        seki_vec = tmp_seki_vec;
    }
    let ans = seki_vec.iter().filter(|seki| **seki == x).count();
    println!("{}", ans);
}



#[allow(dead_code)]
fn product2() {
    input! {
        n: usize, x: usize,
        a_vec: [[usize]; n],
    }

    let ans = dfs(1, 0, n, x, &a_vec);
    println!("{}", ans)
}

fn dfs(seki: usize, posi: usize, n: usize, x: usize, a_vec: &Vec<Vec<usize>>) -> usize {
    let mut cnt = 0;
    if posi == n {
        if seki == x {
            cnt += 1;
        }
        return cnt;
    }

    for ai in &a_vec[posi] {
        if *ai > x / seki { continue; }
        cnt += dfs(seki * ai, posi + 1, n, x, &a_vec);
    }
    cnt
}


#[allow(dead_code)]
fn product() {
    input! {
        n: usize, x: usize,
        a_vec: [[usize]; n],
    }

    let mut seki_vec: Vec<usize> = Vec::new();
    seki_vec.push(1);
    for i in 0..n {
        let mut tmp_seki_vec: Vec<usize> = Vec::new();
        for ai in &a_vec[i] {
            for seki in &seki_vec {
                if *ai > x / *seki { continue; }
                tmp_seki_vec.push(seki * ai);
            }
        }
        seki_vec = tmp_seki_vec;
        // println!("{:?}", seki_vec);
    }
    let ans: usize = seki_vec.iter().filter(|seki| **seki == x).count();
    println!("{}", ans);
} 

#[allow(dead_code)]
fn a_reverse() {
    input! {
        mut l: usize, mut r: usize,
        s: String,
    }
    let mut s_vec: Vec<char> = s.chars().collect();
    while l < r {
        s_vec.swap(l-1, r-1);
        l += 1;
        r -= 1;
    }
    let ans: String = s_vec.iter().map(|c| c.to_string()).collect();
    println!("{}", ans);
}

#[allow(dead_code)]
fn ten_yen_stamp() {
    input! {
        x: usize, y: usize,
    }

    let mut p: usize = 0;
    if x < y {
        if (y - x) % 10 == 0 {
            p = (y - x) / 10;
        } else {
            p = (y - x) / 10 + 1;
        }
    }

    println!("{}", p);
}

#[allow(dead_code)]
fn happy_new_year() {
    input! {
        k: usize,
    }
    let ans: String = get_bin(k).replace("1", "2");
    println!("{}", ans);
}

#[allow(dead_code)]
fn get_bin(mut k: usize) -> String {
    let mut bin_vec: Vec<String> = Vec::new();
    while k > 0 {
        bin_vec.push((k % 2).to_string());
        k /= 2;
    }
    bin_vec.into_iter().rev().collect::<String>()
}

#[allow(dead_code)]
fn longest_segment() {
    input! {
        n: usize,
        xy_vec: [(f64, f64); n],
    }

    let mut len_max: f64 = 0.0;
    for i in 0..n {
        let (x1, y1) = xy_vec[i];
        for j in (i+1)..n {
            let (x2, y2) = xy_vec[j];
            len_max = len_max.max(dist1(x1, y1, x2, y2));
        }
    }
    println!("{}", len_max);
}

fn dist1(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2-x1).powf(2_f64) + (y2-y1).powf(2_f64)).sqrt()
}

#[allow(dead_code)]
fn weird_function() {
    input! {
        t: usize,
    }
    let ans = func(func(func(t)+t) + func(func(t)));
    println!("{}", ans);
}

fn func(t: usize) -> usize {
    t.pow(2) + 2 * t + 3
}

#[allow(dead_code)]
#[fastout]
fn the_kth_time_query() {
    input! {
        n: usize, q: usize,
        a_vec: [usize; n],
        xk_vec: [(usize, usize); q],
    }

    let mut a_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for (index, a) in a_vec.iter().enumerate() {
        a_map.entry(*a).or_insert(vec![]).push(index+1);
    }

    for xk in xk_vec {
        let (x, k) = xk;
        if a_map.contains_key(&x) {
            let v: &Vec<usize> = a_map.get(&x).unwrap();
            if k > v.len() {
                println!("-1");
            } else {
                println!("{}", v[k-1]);
            }
        } else {
            println!("-1");
        }
    }
}

#[allow(dead_code)]
fn climbing_takahashi() {
    input! {
        n: usize,
        h_vec: [usize; n],
    }

    // let ans: usize = *h_vec[0..n-1].iter().max_by(|a, b| a.cmp(b)).unwrap();
    // println!("{}", ans);
    let mut max_h = 0;
    for h in h_vec {
        if max_h < h {
            max_h = h;
        } else {
            break;
        }
    }
    println!("{}", max_h);
} 

#[allow(dead_code)]
fn rotate() {
    input! {
        abc: String,
    }
    let abc_vec: Vec<char> = abc.chars().collect();
    let abc: usize = vec![abc_vec[0], abc_vec[1], abc_vec[2]].iter().collect::<String>().parse::<usize>().ok().unwrap(); 
    let bca: usize = vec![abc_vec[1], abc_vec[2], abc_vec[0]].iter().collect::<String>().parse::<usize>().ok().unwrap();
    let cab: usize = vec![abc_vec[2], abc_vec[0], abc_vec[1]].iter().collect::<String>().parse::<usize>().ok().unwrap();
    let ans: usize = abc + bca + cab;
    println!("{}", ans); 
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