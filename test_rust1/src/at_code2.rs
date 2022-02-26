

fn read <T> () -> T
    where T: std::str::FromStr
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

#[allow(non_snake_case)]
fn main() {
    let vec: Vec<i32> = read::<String>().split_whitespace()
        .map(|s| s.parse().ok().unwrap()).collect();
    let N = vec[0];
    let A = vec[1];
    let B = vec[2];
    let sum: i32 = (0..=N)
        .collect::<Vec<i32>>()
        .iter()
        .filter(|&i| {
            let sum_digit = sum_digit(*i);
            A <= sum_digit && sum_digit <= B
        }).sum();

    println!("{}", sum);
}

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