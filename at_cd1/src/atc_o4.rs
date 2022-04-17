use proconio::{ input, fastout };
use itertools::Itertools;
use std::collections::{ HashMap, HashSet };
use std::cmp::{ min, max };

fn main() {
    round_decimals();
}

/// https://qiita.com/sano192/items/9791b2389d623c1f18d9
fn round_decimals() {
    println!("TODO");
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn key_building() {
    input! {
        N: usize,
        S: [usize; N],
    }

    let mut correct_vec: Vec<usize> = Vec::new();
    for a in 1..1001 {
        for b in 1..1001 {
            // 4ab+3a+3b
            let correct_s = 4*a*b + 3*a + 3*b;
            correct_vec.push(correct_s); 
        }
    }

    let mut mis_cnt = 0;
    for s in S {
        if !correct_vec.contains(&s) {
            mis_cnt += 1;
        }
    }
    
    println!("{}", mis_cnt);
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn last_card2() {
    input! {
        N: usize, mut K: usize, A: usize,
    }

    let mut now = A;
    K -= 1;
    while K > 0 {
        now = (now + 1) % N;
        if now == 0 { now = N; }
        K -= 1;
    }

    println!("{}", now);
}


/// https://qiita.com/sano192/items/7be07fd4d0bb8442dc73
#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn last_card() {
    input! {
        N: usize, K: usize, A: usize,
    }

    let mut ans = (A + K -1) % N;
    if ans == 0 { ans = N; }
    println!("{}", ans);
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn takahashi_secret() {
    input! {
        N: usize, X: usize,
        A: [usize; N],
    }

    let mut known_map: HashSet<usize> = HashSet::new();

    let mut known = X;
    let mut ans = 0;
    for _i in 0..N {
        if known_map.contains(&known) {
            println!("{}", ans);
            return;
        }
        known_map.insert(known);
        known = A[known-1];
        ans += 1;
    }
    println!("{}", ans);
}


/// https://qiita.com/sano192/items/ce8f321bb5aedfa895cd
#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn on_and_off() {
    input! {
        S: i8, T: i8, X: i8, 
    }
    if S < T {
        if S <= X && X < T {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        if S <= X || X < T {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn hard_calculation2() {
    input! {
        mut A: u64, mut B: u64,
    }

    while A > 0 && B > 0 {
        if (A % 10) + (B % 10) >= 10 {
            println!("Hard");
            return;
        }
        A /= 10;
        B /= 10;
        // println!("A: {} B: {}", A, B);
    }
    println!("Easy");
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn hard_calculation() {
    input! {
        A: String, B: String,
    }
    let A_vec: Vec<u64> = A.chars().map(|c| { c.to_string().parse::<u64>().ok().unwrap() }).collect();
    let B_vec: Vec<u64> = B.chars().map(|c| { c.to_string().parse::<u64>().ok().unwrap() }).collect();

    let A_len = A_vec.len();
    let B_len = B_vec.len();
    let len_num = min(A_len, B_len);
    for i in 0..len_num {
        if A_vec[A_len-i-1] + B_vec[B_len-i-1] >= 10 {
            println!("Hard");
            return;
        }
    }
    println!("Easy");
}

/// https://atcoder.jp/contests/abc229/tasks/abc229_a
#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn first_grid() {
    input! {
        S1: String,
        S2: String,
    }

    if S1 == ".#" && S2 == "#." {
        println!("No");
    } else if S1 == "#." && S2 == ".#" {
        println!("No");
    } else {
        println!("Yes");
    }
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn triple_metre2() {
    input! {
        S: String,
    }

    let mut T: String = "".to_string();
    for _i in 0..10 {
        T += "oxx";
    }

    if T.contains(&S) {
        println!("Yes");
    } else {
        println!("No");
    }
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn triple_metre() {
    input! {
        S: String,
    }

    let T: String =  "oxx".repeat(10);
    if T.contains(&S) {
        println!("Yes");
    } else {
        println!("No");
    }
}

/// https://qiita.com/sano192/items/335715b83bd473252783
#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn atcoder_quiz3() {
    input! {
        N: usize,
    }

    let mut num = N;
    if num >= 42 {
        num += 1;
    }

    println!("AGC{:>03}", num);
}


#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn counting2() {
    input! {
        N: isize, Q: isize,
        mut A: [isize; N],
        X: [isize; Q],
    }

    A.sort_by(|a,b| { a.cmp(b) });
    for q in 0..Q {
        let mut left = -1;
        let mut right = N;
        while 1 < right - left {
            let md = (right + left) / 2;
            if X[q as usize] <= A[md as usize] {
                right = md;
            } else {
                left = md;
            }
            // println!("{} {}", left, right);
        }
        println!("{}", N - right);
    }

}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn election2() {
    input! {
        N: usize,
        S: [String; N],
    }

    let mut vote_map: HashMap<String, u8> = HashMap::new();
    for s in S {
        let cnt = vote_map.entry(s).or_insert(0);
        *cnt += 1;
    }

    let kouho = vote_map.iter().max_by_key(|vote| vote.1).unwrap().0;
    println!("{}", kouho);
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn election() {
    input! {
        N: usize,
        S: [String; N],
    }

    let mut vote_map: HashMap<String, u8> = HashMap::new();
    for s in S {
        let cnt = vote_map.entry(s).or_insert(0);
        *cnt += 1;
    }

    let mut max_u = "";
    let mut max_cnt = 0;
    for (k, cnt) in &vote_map {
        if max_cnt < *cnt {
            max_cnt = *cnt;
            max_u = k;
        }
    }
    println!("{}", max_u)
}


/// https://qiita.com/sano192/items/2b2656202b767109387e
#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn water_pressure() {
    input! {
        D: u16,
    }
    let ans = D as f64 / 100_f64;
    println!("{}", ans);
}



#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn graph_isomorphism2() {
    input! {
        N: usize, M: usize,
        AB: [(usize, usize); M],
        CD: [(usize, usize); M],
    }

    // 高橋くんのおもちゃ：つながってるボール
    let mut t_vec: Vec<Vec<usize>> = vec![vec![]; N];
    for (a, b) in AB {
        t_vec[a-1].push(b-1);
        t_vec[b-1].push(a-1);
    }

    // 配列の各要素について順番を小さい順にソート
    // for t in t_vec.iter() {
    //     t.sort_by(|a,b| { a.cmp(b) });
    // }

    // 配列の各要素について順番を小さい順にソート
    let t_vec_sort: Vec<Vec<usize>> = t_vec.into_iter().map(|mut t| {
        t.sort_by(|a,b| { a.cmp(b) });
        t
    }).collect();

    // 青木くんのおもちゃ：つながっているボールの情報
    let mut a_edge_vec: Vec<(usize, usize)> = Vec::new();
    for cd in CD {
        a_edge_vec.push(cd);
    }

    let perm_vec: Vec<usize> = (0..N).collect();
    for perm in perm_vec.iter().permutations(N) {
        // 青木くんのおもちゃ：つながっているボール
        let mut a_vec: Vec<Vec<usize>> = vec![vec![]; N];
        // println!("p: {:?}", perm);
        for (c, d) in &a_edge_vec {
            let c_conv: usize = *perm[c-1];
            let d_conv: usize = *perm[d-1];
            a_vec[c_conv].push(d_conv);
            a_vec[d_conv].push(c_conv);
        }

        // 配列の各要素について順番を小さい順にソート
        let a_vec_sort: Vec<Vec<usize>> = a_vec.into_iter().map(|mut a| { a.sort_by(|a, b| a.cmp(b)); a }).collect();
        // println!("t: {:?}", t_vec_sort);
        // println!("a: {:?}", a_vec_sort);

        // おもちゃが一致したら
        if t_vec_sort == a_vec_sort {
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
        p: [0, 1, 2, 3]
        t: [[1, 2, 3], [0], [0, 3], [0, 2]]
        a: [[2, 3], [2], [0, 1, 3], [0, 2]]

        p: [2, 1, 0, 3]
        t: [[1, 2, 3], [0], [0, 3], [0, 2]]
        a: [[1, 2, 3], [0], [0, 3], [0, 2]]
        Yes
    */
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn graph_isomorphism() {
    input! {
        N: usize, M: usize,
        AB: [(usize, usize); M],
        CD: [(usize, usize); M],
    }

    let mut t_vec: Vec<Vec<bool>> = vec![vec![false; N]; N];
    let mut a_vec: Vec<Vec<bool>> = vec![vec![false; N]; N];

    for (a, b) in AB {
        t_vec[a-1][b-1] = true;
        t_vec[b-1][a-1] = true;
    }

    for (c, d) in CD {
        a_vec[c-1][d-1] = true;
        a_vec[d-1][c-1] = true;
    }

    let perm_vec: Vec<usize> = (0..N).collect();
    for perm in perm_vec.iter().permutations(N) {
        let mut enable = true;
        'outer: for i in 0..N {
            for j in 0..N {
                if t_vec[i][j] != a_vec[*perm[i]][*perm[j]] {
                    enable = false;
                    break 'outer;
                }
            }
        }
        if enable {
            println!("Yes");
            return;
        }
    }

    println!("No");
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn caesar_cipher() {
    input! {
        S: String,
        T: String,
    }

    // 「a」= 97,「b」=98,...「z」= 122
    // a~zまでは26個なので、1個ずつずらす
    for i in 0..=(26-1) {
        let af_vec: Vec<u8> = S.chars().map(|code| { (97 + (code as u8 - 97 + i) % 26) as u8 }).collect();
        let af_string: String = String::from_utf8(af_vec).unwrap();
        if T == af_string {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

// https://qiita.com/sano192/items/3cb3ae52adae8829c94f
#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn qq_solver() {
    input! {
        S: String,
    }
    let tmp_vec: Vec<String> = S.chars().map(|c| { c.to_string() }).collect();
    let ans: i64 = my_parse::<i64>(tmp_vec[0].to_owned()) * my_parse::<i64>(tmp_vec[2].to_owned());
    println!("{}", ans);
}

fn my_parse <T> (s: String) -> T
    where T: std::str::FromStr
{
    s.trim().parse().ok().unwrap()
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn product2() {
    input!{
        N: usize, X: usize,
        A: [[usize]; N],
    }

    let mut seki_vec: Vec<usize> = Vec::new();
    seki_vec.push(1);
    for i in 0..N {
        let mut tmp_seki_vec: Vec<usize> = Vec::new();
        for ai in &A[i] {
            for seki in &seki_vec {
                if ai > &(X / seki) { continue; };
                tmp_seki_vec.push(ai * seki);
            } 
        }
        seki_vec = tmp_seki_vec;
    }

    let ans = seki_vec.iter().filter(|seki| { **seki == X }).count();
    println!("{}", ans);
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn product1() {
    input! {
        N: usize, X: usize,
        A: [[usize]; N], // 先頭が要素数なら要素数省略可
    }
    // println!("{:?}", A);

    //  積=1,袋の番号0からスタート
    let ans = dfs(1, 0, N, X, &A);

    println!("{}", ans);
}

#[allow(non_snake_case)]
fn dfs(seki: usize, pos: usize, N: usize, X: usize, A_vec: &Vec<Vec<usize>>) -> usize {
    let mut ans = 0;
    if pos == N {
        if seki == X {
            ans += 1;
        }
        return ans;
    }

    for ai in &A_vec[pos] {
        // if seki.saturating_mul(*ai) > X { continue };
        if seki > X / ai { continue }; // オーバフローしないように
        ans += dfs(seki * ai, pos + 1, N, X, A_vec);
    }
    ans
}



#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn a_reverse() {
    input! {
        L: usize, R: usize,
        S: String,
    }

    let mut s_vec: Vec<String> = S.chars().map(|c| { c.to_string() }).collect();
    let mut l_pos = L - 1;
    let mut r_pos = R - 1;
    while l_pos < r_pos {
        s_vec.swap(l_pos, r_pos);
        l_pos += 1;
        r_pos -= 1;
    }
    println!("{}", s_vec.join(""));
}

/// https://qiita.com/sano192/items/e54a9f9b994781709881
#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn ten_yen_stamp() {
    input! {
        X: usize, Y: usize,
    }

    if X >= Y {
        println!("0");
    } else {
        println!("{}", ceil(Y-X));
    }
}

fn ceil(x: usize) -> usize {
    (x as f64 / 10_f64).ceil() as usize
}