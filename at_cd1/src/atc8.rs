use proconio::{ input, fastout };

fn main() {
    sorting_by_func();
}


/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_aa
#[allow(dead_code)]
#[allow(non_snake_case)]
#[fastout]
fn sorting_by_func() {
    input! {
        N: usize,
        mut A: [usize; N],
    }
    // A.sort_by(|a, b| {
    //     a.cmp(b)
    // });

    A.sort();

    let ans_vec: Vec<String> = A.iter().map(|a| a.to_string()).collect();

    println!("{}", ans_vec.join(" "));
}

/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_aa
#[allow(dead_code)]
#[allow(non_snake_case)]
#[fastout]
fn sorting() {
    input! {
        N: usize,
        mut A: [usize; N],
    }

    // let mut C: Vec<usize> = vec![0; N];
    let mut C: [usize; 200000] =[0; 200000];

    merge_sort(0, N, &mut A, &mut C);

    for i in 0..N {
        print!("{} ", A[i]);
    }

    println!();
}

#[allow(non_snake_case)]
fn merge_sort(l: usize, r: usize, A: &mut Vec<usize>, C: &mut [usize; 200000]) {

    // println!("cp1");

    if r - l == 1 { return };

    // println!("cp2");

    let mid = (l + r) / 2;
    merge_sort(l, mid, A, C);

    // println!("cp3");

    merge_sort(mid, r, A, C);

    // println!("cp4");

    let mut c1: usize = l as usize;
    let mut c2: usize = mid as usize;
    let mut cnt = 0;

    while c1 != mid || c2 != r {
        if c1 == mid {
            C[cnt] = A[c2];
            c2 += 1;
        } else if c2 == r {
            C[cnt] = A[c1];
            c1 += 1;
        } else {
            if A[c1] <= A[c2] {
                C[cnt] = A[c1];
                c1 += 1;
            } else {
                C[cnt] = A[c2];
                c2 += 1;
            }
        }
        cnt += 1;
    }

    for i in 0..cnt {
        A[l + i] = C[i];
    }
}

#[allow(dead_code)]
#[fastout]
#[allow(non_snake_case)]
fn coin_gacha() {
    input! {
        N: usize,
    }

    // let mut ans: f64 = 0.0;
    // for i in (1..=N).rev() {
    //     ans += (N as f64) / (i as f64);
    // }

    // let ans: f64 = (1..=N).rev().map(|n| {
    //     (N as f64) / (n as f64)
    // }).sum::<f64>();

    // r個から(r+1)個にするのにかかる回数の期待値Pr: N/(N-r)
    // 全体の回数の期待値: SUM{P(0)〜P(N-1)}
    let ans: f64 =  (N as f64) * (0..=N-1).map(|r| {
       1.0 / (N-r) as f64
    }).sum::<f64>();

    println!("{}", ans);
}


/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_y
#[allow(dead_code)]
#[allow(non_snake_case)]
#[fastout]
fn jiro_vacation() {
    input! {
        N: usize,
        A: [f64; N],
        B: [f64; N],
    }

    // let mut ans: f64 = 0.0;
    // for i in 0..N {
    //     ans += (1.0 / 3.0) * A[i] + (2.0 / 3.0) * B[i];
    // }

    let ans: f64 = A.iter().zip(B).map(|(a, b)| {
        (1.0 / 3.0) * a + (2.0 / 3.0) * b
    }).sum::<f64>();
    
    println!("{}", ans);
}


/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_x
#[allow(dead_code)]
#[allow(non_snake_case)]
#[fastout]
fn anser_exam_randomly() {
    input! {
        N: usize,
        PQ: [(f32, f32); N],
    }

    let ans: f32 = PQ.iter().map(|pq| {
        let (p, q) = pq;
        q / p
    }).sum();

    // let mut ans: f64 = 0.0;
    // for pq in PQ {
    //     let (p, q) = pq;
    //     ans += (q as f64) / (p as f64);
    // }
    println!("{}", ans);
}


/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_w
#[allow(dead_code)]
#[allow(non_snake_case)]
#[fastout]
fn dice_expectation() {
    input! {
        N: usize,
        B: [usize; N],
        R: [usize; N],
    }

    let mut blue: f64 = 0.0;
    let mut red: f64 = 0.0;
    for i in 0..(N as usize) {
        blue += B[i] as f64 / N as f64;
        red += R[i] as f64 / N as f64; 
    }
    println!("{}", blue + red);
}


/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_v
#[allow(dead_code)]
#[fastout]
#[allow(non_snake_case)]
fn choose_card3() {
    input! {
        N: u64,
        A: [u64; N],
    }

    let mut cnt: [u64; 100000] = [0; 100000];

    for i in 0..(N as usize) {
        cnt[A[i] as usize] += 1; 
    }

    let mut ans = 0;
    for num in 1..=49999 {
        ans += cnt[num] * cnt[100000 - num];
    }
    ans += (cnt[50000] * (cnt[50000] - 1)) / 2;

    println!("{}", ans);
}



/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_u
#[fastout]
#[allow(non_snake_case)]
fn comb_easy() {
    input! {
        n: usize,
        r: usize,
    }
    
    let nPr = fact(n) / fact(n - r);
    let nCr = nPr / fact(r);
    println!("{}", nCr);
}

#[allow(dead_code)]
fn fact(x: usize) -> usize {
    match x {
        0 | 1 => 1,
        _ => x * fact(x - 1),
    }
}