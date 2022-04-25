use proconio::{ input, fastout };

fn main() {
    choose_elements();
}

#[allow(dead_code)]
fn choose_elements() {
    input! {
        n: usize, k: isize,
        mut a_vec: [isize; n],
        mut b_vec: [isize; n],
    }
    a_vec.insert(0, 0);
    b_vec.insert(0, 0);

    // A又はBを選択して条件を満たすかどうか
    let mut dp: Vec<Vec<bool>> = vec![vec![false; n+1]; 3];
    dp[1][1] = true;
    dp[2][1] = true;
    for j in 2..=n {
        // A側の確認
        if dp[1][j-1] {
            if (a_vec[j-1] - a_vec[j]).abs() <= k {
                dp[1][j] = true;
            }
            if (a_vec[j-1] - b_vec[j]).abs() <= k {
                dp[2][j] = true;
            }
        }
        // B側の確認
        if dp[2][j-1] {
            if (b_vec[j-1] - a_vec[j]).abs() <= k {
                dp[1][j] = true;
            }
            if (b_vec[j-1] - b_vec[j]).abs() <= k {
                dp[2][j] = true;
            }
        }
    }

    // println!("{:?}", dp);

    if dp[1][n] || dp[2][n] {
        println!("Yes");
    } else {
        println!("No");
    }
}

#[allow(dead_code)]
fn mex() {
    input! {
        n: usize,
        a_vec: [usize; n],
    }

    for x in 0..=2000 {
        if !a_vec.iter().any(|a| *a == x) {
            println!("{}", x);
            return;
        }
    }
} 

#[allow(dead_code)]
fn good_morning() {
    input! {
        a: u8, b: u8, c: u8, d: u8,
    }
    if a < c {
        println!("Takahashi");
    } else if a == c && b <= d {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}

#[allow(dead_code)]
fn greatest_common_divisor() {
    input! {
        a: usize, b: usize,
    }
    println!("{}", gcd(a, b));
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

#[allow(dead_code)]
fn factorization() {
    input! {
        n: usize,
    }
    let mut pri = n;
    let lim: usize = (n as f64).sqrt() as usize;
    for x in 2..=lim {
        while pri % x == 0 {
            print!("{} ", x);
            pri /= x;
        }
    }
    if pri >= 2 {
        print!("{}", pri);
    }
}

#[allow(dead_code)]
fn divisor_enumeration() {
    input! {
        n: usize,
    }
    let mut ans_vec: Vec<usize> = Vec::new();
    let lim: usize = (n as f64).sqrt() as usize;
    for x in 1..=lim {
        if n % x == 0 {
            ans_vec.push(x);
            ans_vec.push(n / x);
        }
    }
    ans_vec.sort_by(|a, b| a.cmp(b));
    for ans in ans_vec {
        println!("{}", ans);
    }
}

#[allow(dead_code)]
fn primality_test() {
    input! {
        n: usize,
    }

    let lim: usize = (n as f64).sqrt() as usize;
    for i in 2..=lim {
        if n % i == 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

#[allow(dead_code)]
fn print_prime_numbers() {
    input! {
        n: usize,
    }

    for x in 2..=n {
        if is_prime(x) {
            print!("{} ", x)
        }
    } 
    println!("");
}

#[allow(dead_code)]
fn is_prime(x: usize) -> bool {
    let lim: usize = ((x as f64).sqrt()) as usize;  
    for i in 2..=lim {
        if x % i == 0 {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
fn factorial() {
    input! {
        n: usize,
    }
    println!("{}", fact(n));
}

#[allow(dead_code)]
fn fact(x: usize) -> usize {
    match x {
        0 | 1 => 1,
        _ => x * fact(x-1),
    }
}

#[allow(dead_code)]
fn brute_force2() {
    input! {
        n: usize, s: usize,
        mut a_vec: [usize; n],
    }
    a_vec.insert(0, 0);
    // i番目のカード選んだ時、合計がjとなるかどうか
    let mut dp: Vec<Vec<bool>> = vec![vec![false; s+1]; n+1];
    dp[0][0] = true;
    for i in 1..=n {
        let card_no = a_vec[i];
        for j in 0..=s {
            if j >= card_no {
                // if dp[i-1][j] || (dp[i-1][j-card_no]) {
                //     dp[i][j] = true;
                // }
                dp[i][j] = dp[i-1][j] | dp[i-1][j-card_no];
            } else {
                dp[i][j] = dp[i-1][j];
            }
        }
    }
    println!("{}", yes_or_no(dp[n][s]));
    // println!("{:7?}", dp);
    /*
        3 11
        2 5 9
            合計    0         1       2        3       4         5        6        7        8        9        10       11
            0番目 [true   , false  , false  , false  , false  , false  , false  , false  , false  , false  , false  , false  ]
            1番目 [true  , false  , true   , false  , false  , false  , false  , false  , false  , false  , false  , false  ] 
            2番目 [true  , false  , true   , false  , false  , true   , false  , true   , false  , false  , false  , false  ]
            3番目 [true  , false  , true   , false  , false  , true   , false  , true   , false  , true   , false  , true   ]
    */
}

#[allow(dead_code)]
fn yes_or_no(b: bool) -> String {
    if b {
        String::from("Yes")
    } else {
        String::from("No")
    }
}


#[allow(dead_code)]
fn brute_force1() {
    input! {
        n: usize, s: usize,
    }

    let mut cnt = 0;
    for i in 1..=n {
        for j in 1..=n {
            if i + j <= s {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}


#[allow(dead_code)]
fn number_of_multiples1() {
    input! {
        n: usize, x: usize, y: usize,
    }

    let mut cnt = 0;
    for i in 1..=n {
        if (i % x == 0) || (i % y == 0) {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}

#[allow(dead_code)]
fn print_2() {
    input! {
        n: usize,
    }
    println!("{}", 2 * n + 3);
}

#[allow(dead_code)]
fn modulo_100() {
    input! {
        n: usize,
        a_vec: [usize; n],
    }
    let ans: usize = a_vec.iter().fold(0, |acc, cur| (acc + cur) % 100);
    println!("{}", ans);
}

#[allow(dead_code)]
fn product_of_3_integers() {
    input! {
        a_vec: [usize; 3],
    }
    let ans: usize = a_vec.iter().fold(1, |acc, cur| acc * cur);
    println!("{}", ans); 
}

#[allow(dead_code)]
fn sum_of_n_integers() {
    input! {
        n: usize,
        a_vec: [usize; n],
    }

    println!("{}", a_vec.iter().sum::<usize>());
}

#[allow(dead_code)]
fn sum_of_3_integers() {
    input! {
        a1: usize, a2: usize, a3: usize,
    }
    println!("{}", a1+a2+a3);
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[fastout]
fn print_5n() {
    input! {
        n: u8,
    }
    println!("{}", n+5);
}