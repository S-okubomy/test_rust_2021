use proconio::{ input, fastout };

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Clone, Copy)]
struct Vector {
    vx: f64,
    vy: f64,
}

#[derive(Clone, Copy)]
struct VectorCouple {
    v1: Vector,
    v2: Vector,
}

impl VectorCouple {
    fn new(p1: Point, base_point: Point, p2: Point) -> Self {
        let v1: Vector = Vector{ vx: p1.x - base_point.x, vy: p1.y - base_point.y };
        let v2: Vector = Vector{ vx: p2.x - base_point.x, vy: p2.y - base_point.y };
        VectorCouple{ v1, v2 }
    }

    fn dot(self) -> f64 {
        (self.v1.vx * self.v2.vx) + (self.v1.vy * self.v2.vy) 
    }

    fn cross(self) -> f64 {
        ((self.v1.vx * self.v2.vy) - (self.v1.vy * self.v2.vx)).abs()
    }

    fn get_v1_length(self) -> f64 {
        ((self.v1.vx).powf(2.0) + (self.v1.vy).powf(2.0)).sqrt()
    }

    fn get_v2_length(self) -> f64 {
        ((self.v2.vx).powf(2.0) + (self.v2.vy).powf(2.0)).sqrt()
    }
}


fn main() {
    distance();
}

/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ae
#[allow(dead_code)]
#[allow(non_snake_case)]
#[fastout]
fn distance() {
    input! {
        a: (f64, f64),
        b: (f64, f64),
        c: (f64, f64),
    }

    let a_point = Point{ x: a.0, y: a.1 };
    let base_point = Point{ x: b.0, y: b.1 };
    let c_point = Point{ x: c.0, y: c.1 };

    // ベクトルBA, BC, CA, CBの成分
    let BA_BC = VectorCouple::new(a_point, base_point, c_point);
    let CA_CB = VectorCouple::new(a_point, c_point, base_point);

    let mut ptn = 2;
    if BA_BC.dot() < 0.0 {
        ptn = 1;
    } else if CA_CB.dot() < 0.0 {
        ptn = 3;
    }

    let mut ans: f64 = 0.0;
    if ptn == 1 {
        ans = BA_BC.get_v1_length();
    } else if ptn == 3 {
        ans = CA_CB.get_v1_length();
    } else if ptn == 2 {
        let S = BA_BC.cross();
        let BC_Length = BA_BC.get_v2_length();
        ans = S / BC_Length;
    }

    println!("{}", ans);
}


/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ad
#[allow(dead_code)]
#[allow(non_snake_case)]
#[fastout]
fn binary_serch() {
    input! {
        N: usize,
        X: usize,
        mut A: [usize; N],
    }
    A.insert(0, 0);

    A.sort_by(|a, b| {
        a.cmp(b)
    });

    let mut left = 1;
    let mut right = N;

    while left <= right {
        let mid = (left + right) / 2;
        // println!("chk: {}", mid);
        if A[mid] == X {
            println!("Yes");
            return;
        }

        if A[mid] > X {
            // 探索範囲を前半部分に絞る
            right = mid - 1;
        }

        if A[mid] < X {
            // 探索範囲を後半部分に絞る
            left = mid + 1;
        }
    }

    println!("No");
}