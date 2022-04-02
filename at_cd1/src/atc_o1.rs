use proconio:: { input, fastout };
use std::collections::{ HashMap, HashSet };


fn main() {
    gal_password(); 
}


#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn gal_password() {
    input! {
        N: usize,
    }
    const MOD: isize = 998244353;

    // dp[i][j]: 上からi桁目まで決めて、i桁目の数字がjである組み合わせ数（を 998244353998244353 で割った余り）
    let mut dp: Vec<Vec<isize>> = vec![vec![0; 11]; N+1];

    // 上から1桁目は1個
    for j in 1..=9 {
        dp[1][j] = 1 % MOD;
    }

    for i in 2..=N {
        for j in 1..=9 {
            dp[i][j] = (dp[i-1][j-1] + dp[i-1][j] + dp[i-1][j+1]) % MOD;
        }
    }

    let ans: isize = dp[N].iter().sum::<isize>() % MOD;
    println!("{}", ans);

    // println!("{:2?}", dp);
    // N = 4桁の場合
    //        数字 1   2   3   4   5   5   7   8   9
    // 1桁目 [ 0,  1,  1,  1,  1,  1,  1,  1,  1,  1,  0]
    // 2桁目 [ 0,  2,  3,  3,  3,  3,  3,  3,  3,  2,  0]
    // 3桁目 [ 0,  5,  8,  9,  9,  9,  9,  9,  8,  5,  0]
    // 4桁目 [ 0, 13, 22, 26, 27, 27, 27, 26, 22, 13,  0]
}


/// https://atcoder.jp/contests/abc242/submissions/me
#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn minimize_ordering() {
    input! {
        S: String,
    }
    let mut ans_tmp: Vec<String> = S.chars().map(|c| { c.to_string() }).collect::<Vec<String>>();
    ans_tmp.sort_by(|a, b| { a.cmp(b) });
    let ans = ans_tmp.join("");
    println!("{}", ans);
}


/// https://atcoder.jp/contests/abc242/tasks/abc242_a
#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn t_shirt() {
    input! {
        A: f64, B: f64, C: f64, X: f64,
    }

    let ans: f64;
    if X <= A {
        ans = 1.0;
    } else if A < X && X <= B {
        ans = C / (B - A);
    } else {
        ans = 0.0;
    }

    println!("{}", ans);
}

#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn collision2() {
    input! {
        N: usize,
        XY: [(isize, isize); N],
        S: String,
    }

    // y座標の一覧（重複無し）
    let y_set: HashSet<isize> = XY.iter().map(|(_x, y)| { *y } ).collect::<HashSet<isize>>();
    let dir_vec: Vec<String> = S.chars().map(|c| { c.to_string() }).collect::<Vec<String>>();

    let mut y_dir_R: HashMap<isize, Vec<isize>> = HashMap::new(); // 右向きの同一y座標のx
    let mut y_dir_L = HashMap::new(); // 左向きの同一y座標のx

    for (index, dir) in dir_vec.iter().enumerate() {
        let (x, y) = XY[index];
        match dir.as_str() {
            "R" => {
                y_dir_R.entry(y).or_insert(vec![]).push(x);
            },
            "L" => {
                y_dir_L.entry(y).or_insert(vec![]).push(x);
            },
            _ => (),
        }
    }

    let mut ans: &str = "No";
    for y in y_set {
        // 左向きと右向きの人が同じyにいたら確認する
        if y_dir_R.contains_key(&y) && y_dir_L.contains_key(&y) {
            let min_dir_R: isize = *y_dir_R[&y].iter().min().unwrap();
            let max_dir_L: isize = *y_dir_L[&y].iter().max().unwrap();
            if min_dir_R < max_dir_L {
                ans = "Yes";
                break;
            }
        }
    }

    println!("{}", ans);
}


#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn hit_and_blow() {
    input! {
        N: usize,
        A: [usize; N],
        B: [usize; N],
    }

    // A にも B にも含まれ、その位置も一致している整数の個数
    let mut ans_cnt1: usize = 0;
    for i in 0..N {
        if A[i] == B[i] {
            ans_cnt1 += 1;
        }
    }

    // A にも B にも含まれるが、その位置は異なる整数の個数
    let mut ans_cnt2 = 0;
    for i in 0..N {
        for j in 0..N {
            if (i != j) && (A[i] == B[j]) {
                ans_cnt2 += 1;
            }
        }
    }
    println!("{}", ans_cnt1);
    println!("{}", ans_cnt2);
}

/// https://atcoder.jp/contests/abc243/tasks/abc243_a
#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn shampoo() {
    input! {
        V: isize, A: isize, B: isize, C: isize,
    }

    let v_MOD: isize = V % (A + B + C);

    if v_MOD < A {
        println!("F");
    } else if v_MOD < A + B {
        println!("M");
    } else {
        println!("T");
    }
}



/// https://atcoder.jp/contests/abc244/tasks/abc244_b
#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn go_straight_turn2() {
    input! {
        _N: usize,
        T: String,
    }
    let act_vec: Vec<char> = T.chars().collect::<Vec<char>>();
    let mut dir_no: usize = 1; // 東からスタート
    let dirs: Vec<(isize, isize)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)]; // 北、東、南、西の順
    let mut xy: (isize, isize) = (0, 0);
    for act in act_vec {
        match act.to_string().as_str() {
            "S" => {
                xy.0 += dirs[dir_no].0;
                xy.1 += dirs[dir_no].1; 
            },
            "R" => {
                dir_no = (dir_no + 1) % 4;
            },
            _ => (),
        }
    }

    println!("{} {}", xy.0, xy.1);
}

#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn go_straight_turn() {
    input! {
        _N: usize,
        T: String,
    }

    let act_vec: Vec<String> = T.chars().map(|x| x.to_string()).collect::<Vec<String>>();
    let mut xy: (isize, isize) = (0, 0);
    let mut dir: &str = "e"; // n:北、e:東、s:南、w:西 

    for act in act_vec {
        match dir {
            "n" => {
                if act == "S" {
                    xy.1 += 1;
                } else if act == "R" {
                    dir = "e";
                }
            },
            "e" => {
                if act == "S" {
                    xy.0 += 1;
                } else if act == "R" {
                    dir = "s";
                }
            },
            "s" => {
                if act == "S" {
                    xy.1 -= 1;
                } else if act == "R" {
                    dir = "w";
                }
            },
            "w" => {
                if act == "S" {
                    xy.0 -= 1;
                } else if act == "R" {
                    dir = "n";
                }
            }
            _ => (),
        }
    }
    println!("{} {}", xy.0, xy.1);
}




/// https://atcoder.jp/contests/abc244/tasks
#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn last_letter() {
    input! {
        _N: usize,
        S: String,
    }

    let ans: Vec<char> = S.chars().collect();
    println!("{}", ans.last().unwrap());
}



/// https://atcoder.jp/contests/abc245/tasks/abc245_c
#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn choose_elements() {
    input! {
        N: usize, K: isize,
        mut A: [isize; N],
        mut B: [isize; N],
    }
    A.insert(0, 0);
    B.insert(0, 0);
    let mut dp: Vec<Vec<bool>> = vec![vec![false; N+1]; 3];

    dp[1][1] = true; // Aの初期値
    dp[2][1] = true; // Bの初期値

    for j in 2..=N {
        // 前回のAが条件満たせば、今回確認する。
        if dp[1][j-1] {
            if (A[j-1] -A[j]).abs() <= K {
                dp[1][j] = true; // A側
            }
            if (A[j-1] - B[j]).abs() <= K {
                dp[2][j] = true; // B側
            }
        }

        // 前回のBが条件満たせば、今回確認する。
        if dp[2][j-1] {
            if (B[j-1] - A[j]).abs() <= K {
                dp[1][j] = true;
            }
            if (B[j-1] - B[j]).abs() <= K {
                dp[2][j] = true;
            }
        }
    }

    if dp[1][N] || dp[2][N] {
        println!("{}", yes_or_no(true));
    } else {
        println!("{}", yes_or_no(false))
    }
}

fn yes_or_no(b: bool) -> String {
    if b {
        String::from("Yes")
    } else {
        String::from("No")
    }
}









/// https://atcoder.jp/contests/abc245/tasks/abc245_b
#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn b_mex() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut ans = 0;
    for num in 0..=2000 {
        if !A.iter().any(|a| { *a == num }) {
            ans = num;
            break;
        }
    }

    println!("{}", ans);
}


/// REになるのでNG
/// https://atcoder.jp/contests/abc245/tasks/abc245_b
#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn NG_b_mex2() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut hash_map = HashMap::new();
    for a in A {
        let cnt = hash_map.entry(a).or_insert(0);
        *cnt +=1;
    }

    let mut ans_tmp_vec: Vec<_> = hash_map.iter().collect();
    ans_tmp_vec.sort_by(|a, b| { (a.0).cmp(b.0) } ); // keyの値の小さい順

    // println!("{:?}", hash_map);
    // println!("{:?}", ans_tmp_vec);

    let mut ans = 0;
    for i in 0..=2000 {
        let (key, _) = ans_tmp_vec[i];
        if i != *key {
            ans = i;
            break;
        }
    }

    println!("{}", ans);
}



#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn good_morning() {
    input! {
        A: u8, B: u8, C: u8, D: u8, 
    }

    if A < C {
        println!("Takahashi");
    } else if A == C {
        if B == D {
            println!("Takahashi");
        } else {
            println!("Aoki");
        }
    } else {
        println!("Aoki");
    }
}