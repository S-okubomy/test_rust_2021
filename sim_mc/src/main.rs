use rand::Rng;

const LOOP_MAX: usize = 100_000; // 最大実行回数

trait SimBase {
    fn is_occure(&self, rng: &mut rand::prelude::ThreadRng) -> bool;
}

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

#[derive(Debug)]
struct BlanceSim {
    // saving: Saving,
}

/// モンテカルロ・シミュレーション
/// 【例題】
/// 突然の出費が80%の確率で発生し、その出費は90万円～99万円の間の整数値を取り、
/// 貯金が95万円である場合、貯金が底をつく(貯金の残高が0万円以下)確率は何%か？
/// 【正解】
/// 出費が95万円～99万円の時に貯金が底をつく。
/// 出費があると仮定した場合、95万円～99万円の出費となる確率は50%である。
/// よって、80%×50%＝40%(0.4)の確率で貯金が底をつく。
impl SimBase for BlanceSim {
    fn is_occure(&self, rng: &mut rand::prelude::ThreadRng) -> bool {
        let is_occure: bool = rng.gen_bool(0.8); // 出費が発生するかどうか
        // let is_occure: bool = rng.gen_bool(0.8); // 出費が発生するかどうか
        if is_occure {
            let mut saving = Saving::new(95);
            let expense = rng.gen_range(90..=99); // 出費の額
            // let expense = rng.gen_range(25..=40); // 出費の額
            saving.update_saving(expense);
            if saving.balance <= 0 {
                return true;
            }
        }
        false
    }
}

fn main() {
    let occure_cnt = run(BlanceSim {});
    println!("確率: {}, 残高zero回数: {}, 実行回数: {}"
        , occure_cnt as f32 /LOOP_MAX as f32, occure_cnt, LOOP_MAX);
}

fn run <T> (sim_model: T) -> usize
    where T: SimBase
{
    let mut rng = rand::thread_rng();
    let mut occure_cnt: usize = 0;
    for _rp_cnt in 1..=LOOP_MAX {
        if sim_model.is_occure(&mut rng) {
            occure_cnt = occure_cnt + 1;
        }
    }
    occure_cnt
}
