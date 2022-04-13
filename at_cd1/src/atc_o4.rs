use proconio::{ input, fastout }; 

fn main() {
    qq_solver();
}

fn qq_solver() {
    println!("TODO");
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