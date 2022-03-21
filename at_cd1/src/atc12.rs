use proconio:: { input, fastout };

fn main() {
    intersection();
}

#[derive(Clone, Copy)]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    x: i64,
    y: i64,
}

struct Vector {
    vx: i64,
    vy: i64,
}

struct VectorCouple {
    is_clockwise: bool,  // 時計周りかどうか（ここでは外積が負の場合、時計周りとする）
    is_counter_clockwise: bool, // 反時計周りかどうか
    is_straight_line: bool, // 3つの点が一直線上に並んでいるかどうか
    left_right_point: (Point, Point), // 左端と右端のPoint
}

impl VectorCouple {
    fn new(base_point: Point, p1: Point, p2: Point) -> Self {
        let v1 = Vector { vx: p1.x - base_point.x, vy: p1.y - base_point.y };
        let v2 = Vector { vx: p2.x - base_point.x, vy: p2.y - base_point.y };
        let cross: i64 = (v1.vx * v2.vy) - (v1.vy * v2.vx);
        let is_clockwise: bool = if cross <= 0 { true } else { false };  // 0含む
        let is_counter_clockwise: bool = if cross >= 0 { true } else { false };  // 0含む
        let is_straight_line = if cross == 0 { true } else { false };
        let left_right_point = if base_point <= p1 { (base_point, p1) } else { (p1, base_point) };

        VectorCouple { is_clockwise, is_counter_clockwise, is_straight_line, left_right_point }
    }
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

    let (AB_AC, AB_AD, CD_CA, CD_CB) = setup_inst(A, B, C, D);
    println!("{}", yes_or_no(is_intersect(AB_AC, AB_AD, CD_CA, CD_CB)));
}

#[allow(non_snake_case)]
fn setup_inst(A: (i64, i64), B: (i64, i64), C: (i64, i64), D: (i64, i64)) 
        -> (VectorCouple, VectorCouple, VectorCouple, VectorCouple){
    let A_Point: Point = Point{ x: A.0, y: A.1 };
    let B_Point = Point{ x: B.0, y: B.1 };
    let C_Point = Point{ x: C.0, y: C.1 };
    let D_Point = Point{ x: D.0, y: D.1 };

    let AB_AC = VectorCouple::new(A_Point, B_Point, C_Point);
    let AB_AD = VectorCouple::new(A_Point, B_Point, D_Point);
    let CD_CA = VectorCouple::new(C_Point, D_Point, A_Point);
    let CD_CB = VectorCouple::new(C_Point, D_Point, B_Point);

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
        if AB_AC.is_clockwise && AB_AD.is_counter_clockwise {
            is_AB = true; // 互いに向きが反対なら分ける
        }
        if AB_AC.is_counter_clockwise && AB_AD.is_clockwise {
            is_AB = true;
        }

        let mut is_CD = false; // 線分CDが点A, Bを分けるか
        if CD_CA.is_clockwise && CD_CB.is_counter_clockwise {
            is_CD = true;
        }
        if CD_CA.is_counter_clockwise && CD_CB.is_clockwise {
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