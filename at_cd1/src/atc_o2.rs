use proconio::{ input, fastout };
use std::collections::{ HashMap };
use std::cmp:: { min, max };

fn yes_or_no(b: bool) -> String {
    if b {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

fn main() {
    connect6_3();
}

/// https://atcoder.jp/contests/abc241/tasks/abc241_c
#[allow(non_snake_case)]
// #[fastout] // 何故か使えない
#[allow(dead_code)]
fn connect6_3() {
    input! {
        N: isize,
        S: [String; N],
    }

    // Grid生成
    let grid: Vec<Vec<char>> = S.iter().map(|s| {
         s.chars().collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();

    let mut is_conect: bool = false;
    'outer: for row in 0..N {
        for column in 0..N {
            for dir_enum in Dirs::values() {
                let dir = dir_enum.get_val();
                if is_conect_f3(N, &grid, row, column, &dir) {
                    is_conect = true;
                    break 'outer;
                }
            }
        }
    }
    println!("{}", yes_or_no(is_conect));
}

enum Dirs {
    Horizontal, // 横向き
    Vertical, // 縦向き
    RightObliqueTop, // 右上斜め
    RightObliqueBottom, // 右下斜め
}

impl Dirs {
    fn get_val(&self) -> (isize, isize) {
        // (d_row, d_column)
        match *self {
            Dirs::Horizontal => (0, 1),
            Dirs::Vertical => (1, 0),
            Dirs::RightObliqueTop => (-1, 1),
            Dirs::RightObliqueBottom => (1, 1),
        }
    }

    fn values() -> Vec<Dirs> {
        vec![
            Dirs::Horizontal,
            Dirs::Vertical,
            Dirs::RightObliqueTop,
            Dirs::RightObliqueBottom,      
        ]
    }
}

#[allow(non_snake_case)]
fn is_conect_f3(N: isize, grid: &Vec<Vec<char>>, mut row: isize, mut column: isize
    , dir: &(isize, isize)) -> bool {
    let mut cnt = 0;
    for _ in 1..=6 {
        if !(0 <= min(row, column) && max(row, column) < N) {
            return false;
        }

        if grid[row as usize][column as usize] == '#' {
            cnt += 1;
        }

        row += dir.0;
        column += dir.1;
    }
    return cnt >= 4;
}


/// https://atcoder.jp/contests/abc241/tasks/abc241_c
#[allow(non_snake_case)]
// #[fastout] // 何故か使えない
#[allow(dead_code)]
fn connect6_2() {
    input! {
        N: isize,
        S: [String; N],
    }

    // Grid生成
    let mut grid: Vec<Vec<char>> = Vec::new();
    for s in S {
        let s_vec: Vec<char> = s.chars().collect();
        grid.push(s_vec);
    }
    // println!("{:?}", grid);

    // let dirs: Vec<(isize, isize)> = vec![(1, 0), (0, 1), (1, 1), (1, -1)];
    let dirs: Vec<(isize, isize)> = vec![(0, 1), (1, 0), (1, 1), (-1, 1)]; // (d_row, d_column)

    let mut is_conect: bool = false;
    'outer: for row in 0..N {
        for column in 0..N {
            for dir in &dirs {
                if is_conect_f2(N, &grid, row, column, dir) {
                    is_conect = true;
                    break 'outer;
                }
            }
        }
    }
    println!("{}", yes_or_no(is_conect));
}

#[allow(non_snake_case)]
fn is_conect_f2(N: isize, grid: &Vec<Vec<char>>, mut row: isize, mut column: isize
    , dir: &(isize, isize)) -> bool {
    let mut cnt = 0;
    for _ in 1..=6 {
        if !(0 <= min(row, column) && max(row, column) < N) {
            return false;
        }

        if grid[row as usize][column as usize] == '#' {
            cnt += 1;
        }

        row += dir.0;
        column += dir.1;
    }
    return cnt >= 4;
}

/// https://atcoder.jp/contests/abc241/tasks/abc241_c
#[allow(non_snake_case)]
// #[fastout] // 何故か使えない
#[allow(dead_code)]
fn connect6() {
    input! {
        N: isize,
        S: [String; N],
    }

    // Grid生成
    let mut grid: Vec<Vec<String>> = Vec::new(); // Stringだとcharに比べ、10倍以上遅い、、
    for s in S {
        let s_vec: Vec<String> = s.chars().map(|c| { c.to_string() }).collect::<Vec<String>>();
        grid.push(s_vec);
    }
    // println!("{:?}", grid);

    // let dirs: Vec<(isize, isize)> = vec![(1, 0), (0, 1), (1, 1), (1, -1)];
    let dirs: Vec<(isize, isize)> = vec![(0, 1), (1, 0), (1, 1), (-1, 1)]; // (d_row, d_column)

    let mut is_conect: bool = false;
    'outer: for row in 0..N {
        for column in 0..N {
            for dir in &dirs {
                if is_conect_f(N, &grid, row, column, dir) {
                    is_conect = true;
                    break 'outer;
                }
            }
        }
    }
    println!("{}", yes_or_no(is_conect));
}

#[allow(non_snake_case)]
fn is_conect_f(N: isize, grid: &Vec<Vec<String>>, mut row: isize, mut column: isize
    , dir: &(isize, isize)) -> bool {
    let mut cnt = 0;
    for _ in 1..=6 {
        if !(0 <= min(row, column) && max(row, column) < N) {
            return false;
        }

        if grid[row as usize][column as usize] == "#".to_string() {
            cnt += 1;
        }

        row += dir.0;
        column += dir.1;
    }
    return cnt >= 4;
}



/// https://atcoder.jp/contests/abc241/tasks/abc241_b
#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn pasta() {
    input! {
        N: usize, M: usize,
        A: [usize; N],
        B: [usize; M],
    }

    let mut have_past_cnt_map: HashMap<usize, usize> = HashMap::new();
    let mut use_past_cnt_map: HashMap<usize, usize> = HashMap::new();
    
    for past_len in A {
        let cnt = have_past_cnt_map.entry(past_len).or_insert(0);
        *cnt += 1;    
    }

    for use_past_len in B {
        let use_cnt = use_past_cnt_map.entry(use_past_len).or_insert(0);
        *use_cnt += 1;
    }

    let mut have_past: bool = true;
    for (use_len, use_cnt) in use_past_cnt_map {
        // 指定長さのパスタ持ってれば、個数確認
        if have_past_cnt_map.contains_key(&use_len) {
            let hava_cnt: &usize = have_past_cnt_map.get(&use_len).unwrap();
            if use_cnt > *hava_cnt {
                have_past = false;
                break;
            }
        } else {
            have_past = false;
            break;
        }
    }

    println!("{}", yes_or_no(have_past));
}


/// https://atcoder.jp/contests/abc241/tasks/abc241_a
#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn digit_machine() {
    input! {
        A: [usize; 10],
    }

    let mut ans: usize = 0;
    for _i in 1..=3 {
        ans = A[ans];
    }
    println!("{}", ans);
}