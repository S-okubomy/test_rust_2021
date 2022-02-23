#[allow(dead_code)]
// fn read<T: std::str::FromStr>() -> T {
//     let mut s = String::new();
//     std::io::stdin().read_line(&mut s).ok();
//     s.trim().parse().ok().unwrap()
// }

#[allow(dead_code)]
fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

#[allow(dead_code)]
fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}


// fn main() {
//     let num1 = read::<i32>();
//     println!("read: {}", num1);

//     let num2 = read_vec::<i32>();
//     println!("read2: {:?}", num2);
// }

fn read <T> () -> T
    where T: std::str::FromStr
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

#[allow(dead_code)]
enum Coin {
    Coin50(u16),
    Coin100(u16),
    Coin500(u16),
}

impl Coin {
    fn calc(&self) -> u16 {
        match *self {
            Coin::Coin50(v) => 50 * v,
            Coin::Coin100(v) => 100 * v,
            Coin::Coin500(v) => 500 * v,
        }
    }
}

// use proconio::{input};

#[allow(non_snake_case)]
fn main() {
    let A: u16 = read::<u16>();
    let B: u16 = read::<u16>();
    let C: u16 = read::<u16>();
    let X: u16 = read::<u16>();
    // input! {
    //     A: u16,
    //     B: u16,
    //     C: u16,
    //     X: u16
    // }

    let n = if A < X/500 { A } else { X/500 };
    // println!("500円枚数最大: {}", n);
    let mut cnt = 0;
    for a in 0..=n {
        let m = if B < (X-Coin::Coin500(a).calc()) / 100 { B } else { (X-Coin::Coin500(a).calc()) / 100 };
        // println!("100円枚数最大: {}", m);
        for b in 0..=m {
            // 残金を50で割った値が、所持してる50円枚数以下なら合計金額ピッタリの選び方になるので？
            if (X-Coin::Coin500(a).calc() - Coin::Coin100(b).calc()) / 50 <= C { cnt += 1 }
        } 
    }

    println!("{}", cnt);


    // let n: u8 = read::<u8>();
    // let mut a_vec: Vec<u32> = read::<String>().split_whitespace().map(|s| s.parse().ok().unwrap()).collect();

    // let mut cnt = 0;
    // 'outer: loop {
    //     for i in 0..(n as usize) {
    //         if a_vec[i]%2 != 0 { 
    //             break 'outer;
    //         }
    //         a_vec[i] /= 2;
    //     }
    //     cnt += 1;
    // }

    // println!("{}", cnt);


    // let s_arr = read::<String>();
    // let mut cnt = 0;
    // for s in s_arr.chars() {
    //     match s {
    //         '1' => cnt += 1,
    //         _ =>  (),
    //     }
    //     // if "1" == s.to_string() {
    //     //     cnt += 1;
    //     // }
    // }
    // println!("{}", cnt);

    // let vec :Vec<i32> = read::<String>().split_whitespace().map(|s| s.parse().ok().unwrap()).collect();
    // let a = vec[0];
    // let b = vec[1];
    // if a * b % 2 != 0 {
    //     println!("Odd");
    // } else {
    //     println!("Even");
    // }


    // let a: i32 = read::<i32>();
    // let vec: Vec<i32> = read::<String>().split_whitespace().map(|s| s.parse().unwrap()).collect();
    // let b = vec[0];
    // let c = vec[1];
    // let s = read::<String>();
    // println!("{} {}", a+b+c, s);


    // let mut s = String::new();
    // std::io::stdin().read_line(&mut s).ok();
    // let a = s.trim().parse::<i32>().ok().unwrap();
    // let mut s2 = String::new();
    // let _ = std::io::stdin().read_line(&mut s2);
    // let vec: Vec<i32> = s2.split_whitespace().map(|b| b.parse::<i32>().unwrap()).collect();
    // let b = vec[0];
    // let c = vec[1];
    // let mut s3 = String::new();
    // let _ = std::io::stdin().read_line(&mut s3);
    // let st = s3;
    // println!("{} {} {} {}", a, b, c, st);
}