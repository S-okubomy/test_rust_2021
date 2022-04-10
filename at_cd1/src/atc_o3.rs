use proconio::{ input, fastout };
use std::collections::{ HashSet, HashMap };
use std::cmp::{ min, max };

fn yes_or_no(b: bool) -> String {
    if b {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

fn main() {
    the_kth_time_query();
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn the_kth_time_query() {
    input! {
        N: usize, Q: usize,
        A: [usize; N],
        XK: [(usize, usize); Q],
    }

    let mut A_cnt_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for (index, a) in A.iter().enumerate() {
        A_cnt_map.entry(*a).or_insert(vec![]).push(index+1);
        // let cnt = A_cnt_map.entry(a).or_insert(vec![]).push(index+1);
        // *cnt += 1;
    }

    for xk in XK {
        let (x, k) = xk;
        if A_cnt_map.contains_key(&x) {
            let vec_index: Vec<usize> = A_cnt_map.get(&x).unwrap().to_vec();
            if k <= vec_index.len() {
                println!("{}", vec_index[k-1]);
            } else {
                println!("-1");    
            }
        } else {
            println!("-1");
        }
    }
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn climbing_takahashi() {
    input! {
        N: usize,
        H: [usize; N],
    }

    let mut i = 0;
    let mut ans: usize = 0;
    while (i <= N -1) && (ans < H[i]) {
        ans = H[i];
        i += 1;
    }
    println!("{}", ans);
}

/// https://qiita.com/sano192/items/84e199b2faeae293b02b
#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn rotate() {
    input! { 
        abc: String,
    }

    // let abc: Vec<usize> = abc.chars().map(|c| c.to_string().parse::<usize>().ok().unwrap()).collect::<Vec<usize>>();
    let abc: Vec<String> = abc.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    let (a, b, c) = (abc[0].to_owned(), abc[1].to_owned(), abc[2].to_owned());
    let abc: usize = vec![a.to_owned(), b.to_owned(), c.to_owned()].join("").parse::<usize>().ok().unwrap();
    let bca: usize = vec![b.to_owned(), c.to_owned(), a.to_owned()].join("").parse().ok().unwrap();
    let cab: usize = vec![c, a, b].join("").parse().ok().unwrap();

    let ans: usize = abc + bca + cab;
    
    println!("{}", ans);
    
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn route_map() {
    input! {
        N: usize, M: usize,
        S: [String; N],
        T: [String; M], 
    }

    let map_stop: HashSet<String> = T.into_iter().collect::<HashSet<String>>();

    for i in 0..N {
        if map_stop.contains(&S[i]) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn who_is_missing() {
    input! {
        N: usize,
        A: [usize; 4*N-1],
    }

    // カウント用の配列
    let mut cnt_vec: Vec<usize> = vec![0; N+1];

    for i in 0..(4*N-1) {
        cnt_vec[A[i]] += 1;
    }

    for i in 0..=N {
        if cnt_vec[i] == 3 {
            println!("{}", i);
            return;
        }
    }
}


/// https://qiita.com/sano192/items/65b0d06829c5378cdf0e
#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn chukodai() {
    input! {
        S: String,
        a: usize, b: usize, 
    }
    let mut s_vec: Vec<String> = S.chars().map(|c| { c.to_string() }).collect::<Vec<String>>();
    let a_str: String = s_vec[a-1].to_owned();
    let b_str: String = s_vec[b-1].to_string();
    s_vec[a-1] = b_str;
    s_vec[b-1] = a_str;
    
    println!("{}", s_vec.join(""));
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn kasaka() {
    input! {
        S: String,
    }

    // let mut s_vec: Vec<String> = S.chars().map(|c| { c.to_string() }).collect::<Vec<String>>();
    let s_vec: Vec<char> = S.chars().collect::<Vec<char>>();

    // 先頭から"a"の数を数える
    let mut front_cnt = 0;
    for s in &s_vec {
        if 'a' == *s {
            front_cnt += 1;
        } else {
            break;
        }
    }

    if front_cnt == s_vec.len() {
        println!("Yes");
        return;
    }

    // 後ろから"a"の数を数える
    let mut back_cnt = 0;
    let len_S = s_vec.len();
    // for i in (0..len_S).rev().step_by(2) {
    for i in (0..len_S).rev() {
        // println!("i: {}", i);
        if 'a' == s_vec[i] {
            back_cnt += 1;
        } else {
            break;
        }
    }

    if front_cnt > back_cnt {
        println!("No");
        return;
    }

    // let add_cnt: usize = back_cnt - front_cnt; 
    // for _ in 1..=add_cnt {
    //     s_vec.insert(0, 'a');
    // }

    // println!("{:?}", s_vec);

    // let add_a: String = "a".repeat(back_cnt - front_cnt);
    // let new_s_vec: Vec<String> = (add_a + &S).chars().map(|c| {c.to_string()} ).collect::<Vec<String>>();

    // let len_s = new_s_vec.len();
    // for i in 0..len_s {
    //     if new_s_vec[i] != new_s_vec[len_s-i-1]{
    //         println!("No");
    //         return;
    //     }
    // }

    let len_s = s_vec.len();
    for i in front_cnt..(len_s - back_cnt) {
        if s_vec[i] != s_vec[front_cnt+len_s-back_cnt-i-1]{
            println!("No");
            return;
        }
    }

    println!("Yes");
}

/// https://qiita.com/sano192/items/abea29fc42030300f379
#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn digitnum() {
    input! {
        N: i64,
    }

    /*
    f(1)=1,f(2)=2,...f(9)=9,
    f(10)=1,f(11)=2,...f(99)=90,
    f(100)=1,f(101)=2,f(102)=3,...,f(999)=900
    */

    const MOD: i64 = 998244353;
    let mut ans: i64 = 0;
    for p10 in (1..=18).map(|x| { 10_i64.pow(x) } ) {
        let left: i64 = p10 / 10;
        let right: i64 = min(p10 - 1, N);
        if left <= right {
            let X: i64 = (right - left) + 1;
            ans += triangular_num(X, MOD);
            ans %= MOD;
        }        
    }
    println!("{}", ans);
}

#[allow(non_snake_case)]
fn triangular_num(X: i64, MOD: i64) -> i64 {
    let mod_x: i64 = X % MOD;
    let res:i64 = (mod_x * (mod_x + 1) / 2) % MOD;
    res
}

#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn matrix_transposition() {
    input! {
        H: usize, W: usize,
        A: [[usize; W]; H],
    }

    for x in 0..W {
        for y in 0..H {
            print!("{} ", A[y][x]);
        }
        println!("");
    }
}



/// https://atcoder.jp/contests/abc237/tasks/abc237_a
#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn not_overflow() {
    input! {
        N: i64,
    }

    const M: i64 = 1 << 31;
    // if -2_i64.pow(31) <= N && N < 2_i64.pow(31) {
    if -M <= N && N < M {
        println!("Yes");
    } else {
        println!("No");
    }
}


/// https://atcoder.jp/contests/abc238/tasks/abc238_b
#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn pizza() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut angle_vec: Vec<usize> = Vec::new();

    // はじめの0度
    angle_vec.push(0);
    let mut angle: usize = 0;
    for i in 0..N {
        angle += A[i];
        angle_vec.push(angle % 360);
    }

    // 360度追加
    angle_vec.push(360);

    // 360度超えるかもしれないので、小さい順に並び替え
    angle_vec.sort_by(|a, b| { a.cmp(b) } );

    let mut ans = 0;
    for i in 1..=N+1 {
        ans = max(ans, angle_vec[i] - angle_vec[i-1]);
    }
    println!("{}", ans);
}

/// https://qiita.com/sano192/items/abea29fc42030300f379
#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn exponential_or_quadratic() {
    input! {
        n: usize,
    }

    if 2 <= n && n <= 4 {
        println!("No");
    } else {
        println!("Yes");
    }
}


/// https://atcoder.jp/contests/abc239/tasks/abc239_c
#[allow(non_snake_case)]
// #[fastout]
#[allow(dead_code)]
fn knight_fork() {
    input! {
        x1: isize, y1: isize, x2: isize, y2: isize, 
    }

    let start_x = x1 - 2;
    let end_x = x1 + 2;
    let start_y = y1 - 2;
    let end_y = y1 + 2;

    let mut exist: bool = false;
    'outer: for x in start_x..=end_x {
        for y in start_y..=end_y {
            if dist_sq(x1, y1, x , y) == 5 && dist_sq(x2, y2, x, y) == 5 {
                exist = true;
                break 'outer;
            }
        }
    }

    println!("{}", yes_or_no(exist));
}

fn dist_sq(px: isize, py: isize, x: isize, y: isize) -> isize {
    (px - x).pow(2) + (py - y).pow(2)
} 

/// https://atcoder.jp/contests/abc239/tasks/abc239_b
#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn integer_division() {
    input! {
        X: isize,
    }

    let mut ans = X / 10;
    if X < 0 {
        if X % 10 == 0 {
            ans = X / 10;
        } else {
            ans = X / 10 -1;
        }
    }

    println!("{}", ans);
}

/// https://atcoder.jp/contests/abc239/tasks/abc239_a
/// https://qiita.com/sano192/items/d7fe37db64d946c07581
#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn horizon() {
    input! {
        H: f64,
    }

    let ans = (H * (12800000.0 + H)).sqrt();

    println!("{}", ans)
}

/// https://atcoder.jp/contests/abc240/tasks/abc240_c
#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn jumping_takahashi() {
    input! {
        N: usize, X: usize,
        mut ab: [(usize, usize); N],
    }
    ab.insert(0, (0, 0));

    // dp[i][j]: i回ジャンプして座標jに居るかどうか
    let mut dp: Vec<Vec<bool>> = vec![vec![false; X+1]; N+1];

    dp[0][0] = true;
    for i in 1..=N {
        let (a, b) = ab[i];
        for j in 1..=X {
            if j >= a && dp[i-1][j-a] {
                dp[i][j] = true;
            } else if j >= b && dp[i-1][j-b] {
                dp[i][j] = true;
            }
        }
    }

    // println!("{:7?}", dp);
    println!("{}", yes_or_no(dp[N][X]));

    /*
        2 10
        3 6
        4 5
        座標j    0        1        2        3        4        5        6        7        8        9        10 
        0回目 [true   , false  , false  , false  , false  , false  , false  , false  , false  , false  , false  ]
        1回目 [false  , false  , false  , true   , false  , false  , true   , false  , false  , false  , false  ]
        2回目 [false  , false  , false  , false  , false  , false  , false  , true   , true   , false  , true   ]]
        Yes
    */
}




/// https://atcoder.jp/contests/abc240/tasks/abc240_b
#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn count_distinct_integers() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let tmp_ans: HashSet<usize> = A.into_iter().collect::<HashSet<usize>>();
    println!("{}", tmp_ans.len())
}



/// https://qiita.com/sano192/items/9c32dafea7ce34d67eac
/// https://atcoder.jp/contests/abc240/tasks/abc240_a
#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn edge_chk() {
    input! {
        a: isize, b: isize,
    }

    if b == (a % 10 + 1) || a == (b % 10 + 1) {
        println!("Yes");
    } else {
        println!("No");
    }
}