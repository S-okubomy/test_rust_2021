use proconio::{ input, fastout };
use std::collections::{ HashSet };

fn yes_or_no(b: bool) -> String {
    if b {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

fn main() {
    exponential_or_quadratic();
}

/// https://qiita.com/sano192/items/abea29fc42030300f379
fn exponential_or_quadratic() {
    println!("TODO");
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