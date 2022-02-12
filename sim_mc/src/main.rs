use rand::Rng;

const LOOP_MAX: usize = 100_000; // 最大実行回数 

#[derive(Debug)]
struct Saving {
    balance: i32,
}

impl Saving {
    fn new(balance: i32) -> Self {
        Saving { balance }
    }

    fn update_saving(&mut self, expense: i32) {
        self.balance -= expense;
    }
}

/// モンテカルロ・シミュレーション
/// 【例題】
/// 突然の出費が80%の確率で発生し、その出費は90万円～99万円の間の整数値を取り、
/// 貯金が95万円である場合、貯金が底をつく(貯金の残高が0万円以下)確率は何%か？
/// 【正解】
/// 出費が95万円～99万円の時に貯金が底をつく。
/// 出費があると仮定した場合、95万円～99万円の出費となる確率は50%である。
/// よって、80%×50%＝40%(0.4)の確率で貯金が底をつく。
fn main() {
    let mut rng = rand::thread_rng();

    let mut b_zero_cnt: usize = 0;
    for _rp_cnt in 1..=LOOP_MAX {
        let is_occure: bool = rng.gen_bool(0.8); // 出費が発生するかどうか
        if is_occure {
            let mut saving = Saving::new(95);
            let expense = rng.gen_range(90..=99); // 出費の額
            saving.update_saving(expense);
            if saving.balance <= 0 {
                b_zero_cnt = b_zero_cnt + 1;
                // println!("cnt:{}, {:?}", _rp_cnt, saving);
            }
        }
    }
    println!("確率: {}, 残高zero回数: {}, 実行回数: {}"
        , b_zero_cnt as f32 /LOOP_MAX as f32, b_zero_cnt, LOOP_MAX);
}
