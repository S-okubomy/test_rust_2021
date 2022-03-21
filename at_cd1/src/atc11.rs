use proconio:: { input, fastout };

const INF: f64 = 1.7976931348623157e+308_f64;
const PI: f64 = std::f64::consts::PI;

fn main() {
    intersection();
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    x: i64,
    y: i64,
}

struct Vector {
    vx: i64,
    vy: i64,
}

struct VectorCouple<'a> {
    v1: Vector,
    v2: Vector,
    cross: i64,
    is_clockwise: bool,  // 時計周りかどうか（ここでは外積が負の場合、時計周りとする）
    is_straight_line: bool, // 3つの点が一直線上に並んでいるかどうか
    left_right_point: (&'a Point, &'a Point), // 左端と右端のPoint
}

impl<'a> VectorCouple<'a> {
    fn new(base_point: &'a Point, p1: &'a Point, p2: &Point) -> Self {
        let v1 = Vector { vx: p1.x - base_point.x, vy: p1.y - base_point.y };
        let v2 = Vector { vx: p2.x - base_point.x, vy: p2.y - base_point.y };
        let cross: i64 = (v1.vx * v2.vy) - (v1.vy * v2.vx);
        let is_clockwise: bool = cross < 0;
        let is_straight_line = cross == 0;
        let left_right_point = if base_point <= p1 { (base_point, p1) } else { (p1, base_point) };

        VectorCouple { v1, v2, cross, is_clockwise, is_straight_line, left_right_point }
    }

    // fn cross(&self, v1: Vector, v2: Vector) -> i64 {
    //     (v1.vx * v2.vy) - (v1.vy * v2.vx)
    // }

    // fn is_clockwise(&self, v1: Vector, v2: Vector) -> bool {
    //     self.cross(v1, v2) < 0
    // }
}

/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ah
/// 方針: 条件1と条件2満たしてれば、交差Yes。他1満たしてれば交差Yes。
/// https://github.com/E869120/math-algorithm-book/blob/main/editorial/chap4-1/chap4-1.pdf
/// 条件1: 直線ABは点C, Dを分ける → 外積AB×ACと外積AB×AD の符号比較
///        符号が違う場合、θ(なす角)が互いに逆なので、直線ABを挟んで点Cと点Dが存在
/// 条件2: 直線CDは点A, Bを分ける → 外積CD×CAと外積CD×CB の符号比較
///        符号が違う場合、θ(なす角)が互いに逆なので、直線CDを挟んで点Aと点Bが存在
///   他1: 全ての点が一直線上に並んでいて、区間が重なる場合
#[allow(non_snake_case)]
#[fastout]
fn intersection() {
    input! {
        A: (i64, i64),
        B: (i64, i64),
        C: (i64, i64),
        D: (i64, i64),
    }

    let A_Point = Point{ x: A.0, y: A.1 };
    let B_Point = Point{ x: B.0, y: B.1 };
    let C_Point = Point{ x: C.0, y: C.1 };
    let D_Point = Point{ x: D.0, y: D.1 };

    let AB_AC = VectorCouple::new(&A_Point, &B_Point, &C_Point);
    let AB_AD = VectorCouple::new(&A_Point, &B_Point, &D_Point);
    let CD_CA = VectorCouple::new(&C_Point, &D_Point, &A_Point);
    let CD_CB = VectorCouple::new(&C_Point, &D_Point, &B_Point);

    println!("{}", yes_or_no(is_intersect(AB_AC, AB_AD, CD_CA, CD_CB)));
}

fn setup_inst<'a>(A: (i64, i64), B: (i64, i64), C: (i64, i64), D: (i64, i64)) 
        -> (VectorCouple<'a>, VectorCouple<'a>, VectorCouple<'a>, VectorCouple<'a>){
    let A_Point = Point{ x: A.0, y: A.1 };
    let B_Point = Point{ x: B.0, y: B.1 };
    let C_Point = Point{ x: C.0, y: C.1 };
    let D_Point = Point{ x: D.0, y: D.1 };

    let AB_AC = VectorCouple::new(&A_Point, &B_Point, &C_Point);
    let AB_AD = VectorCouple::new(&A_Point, &B_Point, &D_Point);
    let CD_CA = VectorCouple::new(&C_Point, &D_Point, &A_Point);
    let CD_CB = VectorCouple::new(&C_Point, &D_Point, &B_Point);

    (AB_AC, AB_AD, CD_CA, CD_CB)
}

#[allow(non_snake_case)]
fn is_intersect(AB_AC: VectorCouple, AB_AD: VectorCouple, CD_CA: VectorCouple, CD_CB: VectorCouple) -> bool {
    // 4つの点全てが一直線上にならんでいる場合
    if AB_AC.is_straight_line && AB_AD.is_straight_line
        && CD_CA.is_straight_line && CD_CB.is_straight_line {
            let (a, b) = AB_AC.left_right_point;
            let (c, d) = CD_CA.left_right_point;
            a.max(c) <= b.min(d)
    } else {
        let mut is_AB = false; // 線分ABが点C, Dを分けるか
        if AB_AC.is_clockwise && !AB_AD.is_clockwise {
            is_AB = true; // 互いに向きが反対なら分ける
        }
        if !AB_AC.is_clockwise && AB_AD.is_clockwise {
            is_AB = true;
        }

        let mut is_CD = false; // 線分CDが点A, Bを分けるか
        if CD_CA.is_clockwise && !CD_CB.is_clockwise {
            is_CD = true;
        }
        if !CD_CA.is_clockwise && CD_CB.is_clockwise {
            is_CD = true;
        }

        if is_AB && is_CD {
            true
        } else {
            false
        }
    }
}

fn yes_or_no(b: bool) -> String {
    if b {
        String::from("Yes")
    } else {
        String::from("No")
    }
}



/// https://atcoder.jp/contests/math-and-algorithm/tasks/abc168_c
#[allow(dead_code)]
#[allow(non_snake_case)]
#[fastout]
fn colon() {
    input! {
        A: f64,
        B: f64,
        H: f64,
        M: f64,
    }

    let H_angle = 30.0 * H + 0.5 * M;
    let M_angle = 6.0 * M;
    let Hx = A * (H_angle * PI / 180.0).cos();
    let Hy = A * (H_angle * PI / 180.0).sin();
    let Mx = B * (M_angle * PI / 180.0).cos();
    let My = B * (M_angle * PI / 180.0).sin();

    let dist: f64 = ((Hx - Mx).powf(2.0) + (Hy - My).powf(2.0)).sqrt();
    println!("{}", dist);
}

/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ag
#[allow(dead_code)]
#[fastout]
fn two_circles() {
    input! {
        c1: (f64, f64, f64),
        c2: (f64, f64, f64), 
    }
    let (x1, y1, r1) = c1;
    let (x2, y2, r2) = c2;
    let dist: f64 = ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt();
    
    if dist < (r1 - r2).abs() {
        println!("1");
    } else if dist == (r1 - r2).abs() {
        println!("2");
    } else if dist < r1 + r2 {
        println!("3");
    } else if dist == r1 + r2 {
        println!("4");
    } else {
        println!("5");
    }
}


/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_af
#[allow(dead_code)]
#[allow(non_snake_case)]
#[fastout]
fn nearest_points() {
    input! {
        N: usize,
        mut points: [(f64, f64); N],
    }
    points.insert(0, (0.0, 0.0));

    let mut ans = INF;
    for i in 1..=N {
        let (x1, y1) = points[i];
        for j in i+1..=N {
            let (x2, y2) = points[j];
            let dist = ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt();
            ans = ans.min(dist);
        }
    }
    println!("{}", ans);
}