use proconio:: { input, fastout };
use primal::Sieve;

fn main() {
    sum_of_divisors();
}


/// https://atcoder.jp/contests/math-and-algorithm/tasks/abc172_d
#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn sum_of_divisors() {
    input! {
        N: usize,
    }

    // let ans: usize = (1..=N).map(|K| { K * f_div_cnt(K) }).sum();
    // let ans: usize = get_f_div_arr_timeout4(N).iter().skip(1).map(|(index, f)| { index * f }).sum(); 
    // let mut ans = 0;
    // for (index, f) in get_f_div_arr_timeout(N).iter().enumerate() {
    //     ans += index * f;
    // }
    let ans = get_f_div_arr_timeout(N).iter().enumerate().skip(1).map(|(index, f)| { index * f }).sum::<usize>();
    println!("{}", ans);
}

#[allow(non_snake_case)]
fn get_f_div_arr_timeout(N: usize) -> Vec<usize> {
    let mut F: Vec<usize> = vec![0; N+1];
    for i in 1..=N {
        for j in (i..=N).step_by(i) {
            F[j] += 1;
        }
    }
    F
}

#[allow(non_snake_case)]
#[allow(dead_code)]
fn get_f_div_arr_timeout4(N: usize) -> Vec<(usize, usize)> {
    let mut F: Vec<(usize, usize)> = vec![(0, 0); N+1];
    for i in 1..=N {
        for j in (i..=N).step_by(i) {
            F[j].0 = j;
            F[j].1 += 1;
        }
    }
    F
}

#[allow(dead_code)]
#[allow(non_snake_case)]
fn f_div_cnt_timeout3(K: usize) -> usize {
    let sieve = Sieve::new(10000);
    let fac: Vec<(usize, usize)> = sieve.factor(K).ok().unwrap();

    let mut cnt = 1;
    for f in fac {
        let (_, power) = f;
        cnt *= power+1;
    }
    cnt
}

#[allow(non_snake_case)]
#[allow(dead_code)]
fn f_div_cnt_timeout2(K: usize) -> usize {
    let lim = (K as f64).sqrt() as usize;
    let mut cnt: usize = 0;
    // let mut is_div: Vec<bool> = vec![true; K+1];
    // for i in 2..=lim {
    //     for j in ((2*i)..=K).step_by(i) {
    //         is_div[j] = false;
    //     }
    // }

    for i in 1..=lim {
        if K % i == 0 {
            cnt +=1; // iが約数
            if i * i != K {
                cnt += 1; // K/iも約数
            }
        }
    }
    // println!("{}", cnt);
    cnt
}


#[allow(non_snake_case)]
#[allow(dead_code)]
fn f_div_cnt_timeout1(K: usize) -> usize {
    let mut cnt: usize = 0;
    for i in 1..=K {
        if K % i == 0 {
            cnt +=1;
        }
    }
    cnt
}





/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_k
#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn print_prime_num() {
    input! {
        N: usize,
    }

    let mut prime: Vec<bool> = vec![true; N+1];
    let lim: usize = (N as f64).sqrt() as usize;
    for i in 2..=lim {
        for j in ((2*i)..=N).step_by(i) {
            // println!("{}", j);
            prime[j] = false;
        }
    }

    for i in 2..=N {
        if prime[i] {
            print!("{} ", i)
        }
    }
}

/// https://atcoder.jp/contests/abc165/tasks/abc165_a
#[allow(non_snake_case)]
#[fastout]
#[allow(dead_code)]
fn we_love_golf() {
    input! {
        K: u16,
        A: u16, B: u16,
    }

    if A % K == 0 || A / K != B / K {
        println!("OK");
    } else {
        println!("NG");
    }

    // 別解
    // for x in A..=B {
    //     if x % K == 0 {
    //         println!("OK");
    //         return;
    //     }
    // }
    // println!("NG");
}
