use proconio::{ input, fastout };

fn main() {
    factorial();
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