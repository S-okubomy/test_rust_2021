use proconio:: { input, fastout };
use std::collections::HashMap;

fn main() {
    go_straight_turn2(); 
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
    let mut dir_no: usize = 1;
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

    let mut tmp_vec: Vec<_> = hash_map.iter().collect();
    tmp_vec.sort_by(|a, b| { (a.0).cmp(b.0) } ); // keyの値の小さい順

    // println!("{:?}", hash_map);
    // println!("{:?}", tmp_vec);

    let mut ans = 0;
    for i in 0..=2000 {
        let (key, _) = tmp_vec[i];
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