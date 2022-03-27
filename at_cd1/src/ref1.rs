use proconio::{ input, fastout };

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize, M: usize,
        A: [usize; N],
        B: [usize; M],
    }

    let mut have_past: Vec<bool> = vec![true; N];

    let mut ans = "Yes";
    for b in B {
        let mut is_ok = false;
        for (index, a) in A.iter().enumerate() {
            if have_past[index] && b == *a {
                have_past[index] = false;
                is_ok = true;
                break;
            }
        }
        if !is_ok {
            ans = "No";
            break;
        }
    }
    println!("{}", ans);
}







// fn main() {
//     let numbers = read_numbers();
//     if let [n] = numbers[..] {
//         solve(n);
//     }
// }

fn solve(n: u64) {
    let f = fac(n);
    println!("{}", f);
}

fn fac(n: u64) -> u64 {
    fac1(n, 1)
}

fn fac1(n: u64, acc: u64) -> u64 {
    if n == 0 {
        acc
    } else {
        fac1(n - 1, n * acc)
    }
}

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line
}

fn read_numbers() -> Vec<u64> {
    read_line()
        .trim()
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect()
}

fn yesno(b: bool) -> String {
    if b {
        String::from("Yes")
    } else {
        String::from("No")
    }
}



// use proconio::{fastout, input};
 
// #[fastout]
// fn main() {
//     input! {
//         n: usize
//     }
 
//     println!("{}", factorial(n));
// }
 
// fn factorial(n: usize) -> usize {
//     if n != 0 {
//         factorial(n - 1) * n
//     } else {
//         1
//     }
// }



// fn main() {
  
//     proconio::input! {
//       n: u64
//     }
    
//     println!("{}", factorial(n));
//   }
   
//   fn factorial(a: u64) -> u64 {
//     match a {
//       0 | 1 => 1,
//       _ => a * factorial(a - 1)
//     }
//   }