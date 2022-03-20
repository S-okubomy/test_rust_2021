use proconio::{ input, fastout };

fn main() {
    jiro_vacation();
}

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