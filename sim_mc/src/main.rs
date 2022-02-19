use rand::Rng;
use std::thread;
use std::sync::mpsc;

const N_WORKERS: i32 = 4; // スレッド数
const LOOP_MAX: usize = 100_000; // 最大実行回数 ※スレッド数（N_WORKERS）で割り切れる値を設定する。
// const LOOP_MAX: usize = 8; // 最大実行回数

trait SimBase {
    fn init(&mut self);
    fn is_occure(&mut self, rng: &mut rand::prelude::ThreadRng) -> bool;
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

/// モンテカルロ・シミュレーション
/// 【例題】
/// 突然の出費が80%の確率で発生し、その出費は90万円～99万円の間の整数値を取り、
/// 貯金が95万円である場合、貯金が底をつく(貯金の残高が0万円以下)確率は何%か？
/// 【正解】
/// 出費が95万円～99万円の時に貯金が底をつく。
/// 出費があると仮定した場合、95万円～99万円の出費となる確率は50%である。
/// よって、80%×50%＝40%(0.4)の確率で貯金が底をつく。
#[derive(Debug)]
struct BlanceSim {
    init_money: i32,
}

impl SimBase for BlanceSim {
    fn is_occure(&mut self, rng: &mut rand::prelude::ThreadRng) -> bool {
        let is_occure: bool = rng.gen_bool(0.8); // 出費が発生するかどうか
        // let is_occure: bool = rng.gen_bool(0.8); // 出費が発生するかどうか
        if is_occure {
            let mut saving = Saving::new(self.init_money);
            let expense = rng.gen_range(90..=99); // 出費の額
            // let expense = rng.gen_range(25..=40); // 出費の額
            saving.update_saving(expense);
            if saving.balance <= 0 {
                return true;
            }
        }
        false
    }
    fn init(&mut self) { }
}

/// モンテカルロ・シミュレーション
/// 【例題】
/// １ヶ月の収入が25万円〜40万円、支出が20万円〜35万の場合、
/// 30年間で貯金が2000万円超えとなる確率は何%か？
/// 【正解】
/// ---
#[derive(Debug)]
struct LifeDepositSim {
    init_deposit_amount: i32,
    deposit_amount: i32,
}

impl SimBase for LifeDepositSim {
    fn init(&mut self)
    {
        self.deposit_amount = self.init_deposit_amount;
    }

    fn is_occure(&mut self, rng: &mut rand::prelude::ThreadRng) -> bool {
        const LIFE_PERIOD: i32 = 30; // 30年間
        for _y_cnt in 1..=LIFE_PERIOD {
            for _m_cnt in 1..12 { // 1年==12ヶ月
                let income: i32 = rng.gen_range(27..=45);
                let expense: i32 = rng.gen_range(25..=35);
                let balance = income - expense;
                self.deposit_amount += balance; 
                // println!("経過年:{} {} {} {}", _y_cnt, income, expense, self.deposit_amount);
            }
        }
        if self.deposit_amount > 2000 { return true }
        false
    }
}


fn main() {    

    // let occure_cnt = run(BlanceSim { init_money : 95 });
    // let occure_cnt = run(LifeDepositSim {init_deposit_amount: 95, deposit_amount: 0});
    let occure_cnt = thread_process();

    println!("確率: {}, 発生回数: {}, 実行回数: {}"
        , occure_cnt as f32 /LOOP_MAX as f32, occure_cnt, LOOP_MAX);
}

fn run <T> (mut sim_model: T) -> usize
    where T: SimBase
{
    let mut rng = rand::thread_rng();
    let mut occure_cnt: usize = 0;
    for _rp_cnt in 1..=LOOP_MAX {
        sim_model.init();
        if sim_model.is_occure(&mut rng) {
            occure_cnt = occure_cnt + 1;
        }
    }
    occure_cnt
}

fn thread_process() -> usize {
    let thread_loop_max = LOOP_MAX/(N_WORKERS as usize);
    println!("m: {}", thread_loop_max);

    let (tx, rx) = mpsc::channel();
    for _num in 1..=N_WORKERS {
        // ここの tx は for が回るたびにスコープから抜ける => drop する
        let tx = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            let occure_cnt = run_thread(LifeDepositSim {init_deposit_amount: 95, deposit_amount: 0}, thread_loop_max);
            tx.send(occure_cnt).unwrap();
        });
    }

    drop(tx); // 明示的にdropする必要あり

    let sum_occure_cnt: usize = rx.iter().take(N_WORKERS as usize).map(|val| val).sum();
    sum_occure_cnt
}

fn run_thread <T> (mut sim_model: T, thread_loop_max: usize) -> usize
    where T: SimBase
{
    let mut rng = rand::thread_rng();
    let mut occure_cnt: usize = 0;
    for _rp_cnt in 1..=thread_loop_max {
        sim_model.init();
        if sim_model.is_occure(&mut rng) {
            occure_cnt = occure_cnt + 1;
        }
    }
    occure_cnt
}
