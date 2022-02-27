// use proconio::input;

fn read <T> () -> T
    where T: std::str::FromStr
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

#[allow(non_snake_case)]
fn main() {
    let TMP_S: String = read::<String>();
    let mut S = &TMP_S[..];
    let pat_arr: [&str; 4] = ["dream", "dreamer", "erase", "eraser"];

    let mut can: bool = true;
    while S.len() > 0 {
        // let matched = pat_arr.iter().find(|&p| S.ends_with(p));
        // if let Some(p) = matched {
        //     S = &S[..(S.len()-p.len())];
        // } else {
        //     can = false;
        //     break;
        // }

        match pat_arr.iter().find(|&p| S.ends_with(p)) {
            Some(p) => S = &S[..(S.len()-p.len())], // ends_withで後ろから比較して、一致してたら前側を残す
            None => {
                can = false;
                break;
            }
        }
    }

    println!("{}", if can { "YES" } else { "NO" });
}

// fn main() {
//     let vec: Vec<u32> = read::<String>().split_whitespace().map(|s| s.trim().parse().ok().unwrap()).collect();
//     let N = vec[0];
//     let Y = vec[1];

//     let mut is_nothing: bool = true;
//     'outer: for i in 0..=N {
//         for j in 0..=N-i {
//             let k = N - i - j;
//             let sum = 10000 * i + 5000 * j + 1000 * k;
//             if Y == sum {
//                 println!("{} {} {}", i, j, k);
//                 is_nothing = false;
//                 break 'outer;
//             }
//         }
//     }

//     if is_nothing { println!("-1 -1 -1") }
// }

// fn main() {
//     let N: u8 = read::<u8>();
//     let mut vec: Vec<i8> = Vec::new();
//     for _i in 0..(N as usize) {
//         vec.push(read::<i8>());
//     }
//     vec.sort_by_key(|k| -k); //降順に並び替え
//     let mut cnt = 1;
//     for i in 1..(N as usize) {
//         if vec[i - 1] > vec[i] {
//             cnt += 1;
//         }
//     }

//     println!("{}", cnt);
// }


// fn main() {
//     read::<u16>();
//     let mut vec: Vec<i16> = read::<String>().split_whitespace().map(|s| s.parse().ok().unwrap()).collect();
//     vec.sort_by_key(|k| -k);  //数の大きいものから取った方が点数高いので降順に並び変える
//     let res: (i16, i16, bool) = vec.into_iter().fold((0, 0, true), | acc, cur | {
//         // acc.0: Alice,  acc.1: Bob, acc.2: どっちが取るか
//         if acc.2 { (acc.0 + cur, acc.1, false) } else { (acc.0, acc.1 + cur, true) }
//     });
//     println!("{}", res.0 - res.1);
// }


// fn main() {
//     let vec: Vec<i32> = read::<String>().split_whitespace()
//         .map(|s| s.parse().ok().unwrap()).collect();
//     let N = vec[0];
//     let A = vec[1];
//     let B = vec[2];

//     // input! {
//     //     N: i32,
//     //     A: i32,
//     //     B: i32,
//     // }

//     let sum: i32 = (0..=N)
//         .collect::<Vec<i32>>()
//         .iter()
//         .filter(|&i| {
//             let sum_digit = sum_digit(*i);
//             A <= sum_digit && sum_digit <= B
//         }).sum();

//     println!("{}", sum);
// }

#[allow(dead_code)]
fn sum_digit(n: i32) -> i32 {
    n.to_string().chars().map(|i| {
        i.to_digit(10).unwrap() as i32
    }).sum()
}






// fn main() {
//     let vec: Vec<i32> = read::<String>().split_whitespace().map(|s| s.parse().ok().unwrap()).collect();
//     let N = vec[0];
//     let A = vec[1];
//     let B = vec[2];
//     let mut sum = 0;
//     for n in 1..=N {
//         if n == 10000 {
//             sum += 10000;
//             break;
//         }
//         let digit_sum = n/1000 + (n%1000)/100 + (n%100)/10 + n%10; // 各桁の和
//         // println!("各桁合計:{}, 千の位:{}, 百の位:{}, 十の位:{}, 一の位:{}", digit_sum, n/1000, (n%1000)/100, (n%100)/10, n%10);
//         if A <=digit_sum && digit_sum <=B {
//             sum += n;
//         } 
//     }

//     println!("{}", sum);
// }