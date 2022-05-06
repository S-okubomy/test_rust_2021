use proconio::{ input, fastout };
use std::collections::{ HashMap, HashSet };
use itertools::Itertools;

fn main() {
    many_oranges();
}

#[allow(dead_code)]
fn many_oranges() {
    input! {
        a: usize, b: usize, w :usize,
    }
    let wg: usize = 1000 * w;
    let mut min_ans: usize = std::usize::MAX;
    let mut max_ans: usize = std::usize::MIN;
    // println!("{} {}", min_ans, max_ans);
    for x in 1..=10_usize.pow(6) {
        // 【A * みかんの個数x <= W <= B * みかんの個数x】 に入っていれば、みかんの重さは少数もありえるため、
        // 合計した重さがWになる可能性があるということ？
        // println!("x {} {} {}", x, a * x, b * x);
        if a * x <= wg && wg <= b * x {
            // println!("{} {} {}", x, a * x, b * x);
            min_ans = min_ans.min(x);
            max_ans = max_ans.max(x);
        }
    }
    if min_ans == std::usize::MAX {
        println!("UNSATISFIABLE");
    } else {
        println!("{} {}", min_ans, max_ans);
    }
}

#[allow(dead_code)]
fn ipfl() {
    input! {
        n: usize,
        s: String,
        q: usize,
        tab_vec: [(usize, usize, usize); q],
    }
    let mut s_vec: Vec<char> = s.chars().collect();
    s_vec.insert(0, 'x');

    let mut is_flip: bool = false; // 反転しているかどうか
    for (t, a, b) in tab_vec {
        if t == 1 {
            let a_ind;
            let b_ind;
            if is_flip {
                if a <= n {
                    a_ind = a + n;
                } else {
                    a_ind = a - n;
                }
                if b <= n {
                    b_ind = b + n;
                } else {
                    b_ind = b - n;
                }
            } else {
                a_ind = a;
                b_ind = b;
            }
            s_vec.swap(a_ind, b_ind);
        } else {
            if is_flip {
                is_flip = false;
            } else {
                is_flip = true;
            }
        }
    }

    let s_left: String = s_vec[1..=n].iter().map(|c| c.to_string()).collect();
    let s_rigth: String = s_vec[n+1..=2*n].iter().map(|c| c.to_string()).collect();
    let ans: String;
    if is_flip {
        ans = s_rigth + &s_left;
    } else {
        ans = s_left + &s_rigth;
    }
    println!("{}", ans);

    // if is_flip {
    //     let mut flip_vec: Vec<char> = Vec::new();
    //     for i in n+1..=2*n {
    //         flip_vec.push(s_vec[i]);
    //     }
    //     for i in 1..=n {
    //         flip_vec.push(s_vec[i]);
    //     }
    //     let ans: String = flip_vec.iter().collect();
    //     println!("{}", ans);
    // } else {
    //     let ans: String = s_vec.iter().skip(1).collect();
    //     println!("{}", ans);
    // }
}

#[allow(dead_code)]
fn walking_takahashi() {
    input! {
        x: isize, k: isize, d: isize,
    }
    let x_abs = x.abs();
    // if x_abs - d * k >= 0 {
    if x_abs / k - d >= 0 {
        println!("{}", x_abs - d * k);
    } else {
        // 0を飛び越える直前
        let j_bef = x_abs - (x_abs / d) * d;
        // 0を飛び超えた直前 座標
        let j_af = j_bef - d;
        // 残りの回数
        let rem_k = k - x_abs / d;

        if rem_k % 2 == 0 {
            println!("{}", j_bef.abs());
        } else {
            println!("{}", j_af.abs());
        }
    }
}



#[allow(dead_code)]
fn wa_sum_pairs() {
    input! {
        n: usize,
        a_vec: [usize; n],
    }

    const MOD: usize = 1000000007;
    let mut a_sum: usize = a_vec.iter().sum();

    let mut ans = 0;
    for i in 0..n {
        a_sum -= a_vec[i];
        ans += (a_vec[i] * a_sum) % MOD;
        // println!("{} x {}", a_vec[i], a_sum);
    }
    println!("{}", ans % MOD);
}

#[allow(dead_code)]
fn unexpressed() {
    input! {
        n: usize,
    }
    let lim = (n as f64).sqrt() as usize;
    let mut able_set: HashSet<usize> = HashSet::new();
    for a in 2..=lim {
    // for a in 2..=n {
        let mut x = a * a;
        while x <= n {
            able_set.insert(x);
            x *= a;
        }
    }
    println!("{}", n - able_set.len());
}


#[allow(dead_code)]
fn over_unexpressed() {
    input! {
        n: usize,
    }
    let mut able_set: HashSet<usize> = HashSet::new();
    for a in 2..=10_usize.pow(5) {
        for b in 2..=33 {
            if a.pow(b) <= n {
                able_set.insert(a.pow(b));
            }
        }
    }
    println!("{}", n - able_set.len());
}

#[allow(dead_code)]
fn travel() {
    input! {
        n: usize, k: usize,
        t_vec: [[usize; n]; n],
    }
    let perm_vec: Vec<usize> = (0..n).collect();
    let mut cnt = 0;
    for perm in perm_vec.iter().permutations(n) {
        let mut sum = 0;
        for i in 0..n-1 {
            if *perm[0] != 0 { continue; } 
            // println!("{:?}", perm);
            let now = *perm[i];
            let to = *perm[i+1];
            sum += t_vec[now][to];
        }
        let last = *perm[n-1];
        sum += t_vec[last][0]; // 都市1に戻る
        // println!("{}", sum);
        if k == sum {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}

#[allow(dead_code)]
fn to3_2() {
    input! {
        n: usize,
    }

    let n_vec: Vec<usize> = get_digit_end(n);
    let n_vec_len = n_vec.len();
    let mut cnt_vec: Vec<usize> = vec![0; 3];
    for n in n_vec {
        cnt_vec[n % 3] += 1;
    }

    if n % 3 == 0 {
        println!("0");
    } else if n % 3 == 1 {
        if cnt_vec[1] >= 1 {
            if n_vec_len <= 1 {
                println!("-1");
            } else {
                println!("1");
            }
        } else {
            if n_vec_len <= 2 {
                println!("-1");
            } else {
                println!("2");
            }
        }
    } else if n % 3 == 2 {
        if cnt_vec[2] >= 1 {
            if n_vec_len <= 1 {
                println!("-1");
            } else {
                println!("1");
            }
        } else {
            if n_vec_len <= 2 {
                println!("-1");
            } else {
                println!("2");
            }
        }
    }
}

fn get_digit_end(mut x: usize) -> Vec<usize> {
    let mut v: Vec<usize> = Vec::new();
    while x > 0 {
        v.push(x % 10);
        x /= 10;
    }
    v.into_iter().rev().collect()
}

#[allow(dead_code)]
fn to3() {
    input! {
        n: String,
    }
    let n_vec: Vec<usize> = n.chars().map(|c| c.to_string().parse::<usize>().ok().unwrap()).collect();
    let n_vec_len = n_vec.len();
    let sum_n_vec: usize = n_vec.iter().sum();
    let mut cnt_vec: Vec<usize> = vec![0; 3];
    for n in n_vec {
        cnt_vec[n%3] += 1;
    }

    if sum_n_vec % 3 == 0 {
        println!("0");
    } else if sum_n_vec % 3 == 1 {
        if cnt_vec[1] > 0 {
            if n_vec_len == 1 {
                println!("-1");
            } else {
                println!("1");
            }
        } else {
            if n_vec_len == 2 {
                println!("-1");
            } else {
                println!("2");
            }
        }
    } else if sum_n_vec % 3 == 2 {
        if cnt_vec[2] > 0 {
            if n_vec_len == 1 {
                println!("-1");
            } else {
                println!("1");
            }
        } else {
            if n_vec_len == 2 {
                println!("-1");
            } else {
                println!("2");
            }
        }
    }

}

#[allow(dead_code)]
fn collinearity() {
    input! {
        n: usize,
        xy_vec: [(isize, isize); n],
    }

    for i in 0..n {
        let (x1, y1) = xy_vec[i];
        for j in i+1..n {
            let (x2, y2) = xy_vec[j];
            for k in j+1..n {
                let (x3, y3) = xy_vec[k];
                if (y3-y1) * (x2-x1) == (y2-y1) * (x3-x1) {
                // if (y3-y1) / (y2-y1)  ==  (x3-x1) / (x2-x1) {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}

#[allow(dead_code)]
fn alcoholic() {
    input! {
        n: usize, x: usize,
        vp_vec: [(usize, usize); n],
    }

    let mut sum: usize = 0;
    let mut cnt = 0;
    for (v, p) in vp_vec {
        cnt += 1;
        sum += v * p;
        if sum > 100 * x {
            println!("{}", cnt);
            return;
        }
    }
    println!("-1");
}


#[allow(dead_code)]
fn snack() {
    input! {
        a: usize, b: usize,
    }
    let ans = (a * b) / gcd(a, b);
    println!("{}", ans);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

#[allow(dead_code)]
fn travel_costs() {
    input! {
        n: usize, k: usize,
        mut ab_vec: [(usize, usize); n],
    }
    ab_vec.sort_by(|a, b| a.0.cmp(&b.0));
    let mut now_v = k;
    for (a, b) in ab_vec {
        if a <= now_v {
            now_v += b;
        }
    }
    println!("{}", now_v);
}

#[allow(dead_code)]
fn tle_travel_costs() {
    input! {
        n: usize, mut k: isize,
        mut ab_vec: [(isize, isize); n],
    }
    ab_vec.sort_by(|a, b| a.0.cmp(&b.0));
    // println!("{:?}", ab_vec);
    let mut ab_map: HashMap<isize, isize> = HashMap::new();
    for (a, b) in ab_vec.iter() {
        let cnt = ab_map.entry(*a).or_insert(0);
        *cnt += b;
    }
    
    let mut map_sort: Vec<(isize, isize)> = ab_map.iter().map(|ab| (*ab.0, *ab.1)).collect();
    map_sort.sort_by(|a, b| a.0.cmp(&b.0));
    // println!("{:?}", map_sort);
    
    let mut cnt = 0;
    let mut stop_vi = 0;
    while k > 0 {
        cnt += k;
        k = 0;
        if ab_map.contains_key(&cnt) {
            k += ab_map.get(&cnt).unwrap();
            stop_vi = cnt;
        } else {
            for (a, b) in &map_sort {
                if stop_vi < *a && *a < cnt {
                    k += b;
                    stop_vi = *a;
                }
            }
        }
    }
    println!("{}", cnt);
}

#[allow(dead_code)]
fn cream_puff() {
    input! {
        n: usize,
    }

    let lim = (n as f64).sqrt() as usize;
    let mut ans_vec: Vec<usize> = Vec::new();
    for x in 1..=lim {
        if n % x == 0 {
            ans_vec.push(x);
            if x != n / x {
                ans_vec.push(n/x);
            }
        } 
    }
    ans_vec.sort_by(|a, b| a.cmp(b));
    for ans in ans_vec {
        println!("{}", ans);
    }
}

#[allow(dead_code)]
fn unlucky7() {
    input! {
        n: usize,
    }
    // println!("{}", cnv_oct(n));
    let mut cnt = 0;
    for x in 1..=n {
        if !x.to_string().contains("7") && !cnv_oct(x).contains("7") {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}

fn cnv_oct(mut x: usize) -> String {
    let mut oct_vec: Vec<String> = Vec::new();
    while x > 0 {
        oct_vec.push((x % 8).to_string());
        x /= 8;
    }
    oct_vec.reverse();
    oct_vec.into_iter().collect::<String>()
}

#[allow(dead_code)]
fn kaprekar_number() {
    input! {
        n: usize, k: usize,
    }
    let mut a = n;
    for _i in 1..=k {
        a = func(a.to_string());
    }
    println!("{}", a);
}

fn func(s: String) -> usize {
    let mut s_vec: Vec<usize> = s.chars().map(|c| c.to_string().parse::<usize>().ok().unwrap()).collect();
    s_vec.sort_by(|a, b| b.cmp(&a));
    let g1: usize = s_vec.iter().map(|s| s.to_string()).collect::<String>().parse::<usize>().ok().unwrap();
    s_vec.sort_by(|a, b| a.cmp(&b));
    let g2: usize = s_vec.iter().map(|s| s.to_string()).collect::<String>().parse::<usize>().ok().unwrap();
    // println!("{} {}" , g1, g2);
    g1- g2
}

#[allow(dead_code)]
fn visibility() {
    input! {
        h: isize, w: isize, x: isize, y: isize,
        s_vec: [String; h],
    }
    let s_map: Vec<Vec<char>> = s_vec.iter().map(|s| s.chars().collect()).collect();
    // println!("{:?}", s_map);
    let dir_vec: Vec<(isize, isize)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut cnt = 0;
    for dir in dir_vec {
        cnt += find(h, w, x-1, y-1, dir, &s_map);
    }
    
    println!("{}", cnt-3)
}

fn find(h: isize, w: isize, mut i: isize, mut j: isize, dir: (isize, isize), s_vec: &Vec<Vec<char>>) -> usize {
    let mut cnt = 0;
    while (0 <= i && i < h) && (0 <= j && j < w) {
        // println!("{} {}", i, j);
        if s_vec[i as usize][j as usize] == '#' {
            return cnt;
        }
        cnt += 1;
        j += dir.0;
        i += dir.1;
    } 
    cnt
}

#[allow(dead_code)]
fn highest_mountain() {
    input! {
        n: usize,
        mut st_vec: [(String, usize); n],
    }
    st_vec.sort_by(|a, b| b.1.cmp(&a.1));
    println!("{}", st_vec[1].0);
}

#[allow(dead_code)]
fn magic3() {
    input! {
        n: usize, s: usize, d: usize,
        xy_vec: [(usize, usize); n],
    }

    for (x, y) in xy_vec {
        if x < s && d < y {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

#[allow(dead_code)]
fn remove_it() {
    input! {
        n: usize, x: usize,
        a_vec: [usize; n],
    }
    let rm_a_vec: Vec<usize> = a_vec.into_iter().filter(|a| *a != x).collect();
    for rm_a in rm_a_vec {
        print!("{} ", rm_a);
    }
    println!("");
}

#[allow(dead_code)]
fn orthogonality() {
    input! {
        n: usize,
        a_vec: [isize; n],
        b_vec: [isize; n],
    }

    let mut seki = 0;
    for i in 0..n {
        seki += a_vec[i] * b_vec[i]; 
    }
    println!("{}", if seki == 0 { "Yes" } else { "No" });
}

#[allow(dead_code)]
fn rotate() {
    input! {
        s: String,
    }
    let s_vec: Vec<char> = s.chars().collect();
    let ans: String = vec![s_vec[1], s_vec[2], s_vec[0]].iter().collect();
    println!("{}", ans);
}


#[allow(dead_code)]
fn square_inequality() {
    input! {
        a: usize, b: usize, c: usize,
    }
    println!("{}", if a.pow(2) + b.pow(2) < c.pow(2) { "Yes" } else { "No" });
}
