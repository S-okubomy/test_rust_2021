use std::{env};
use rand::Rng;
use std::thread;
use std::sync::mpsc;
use std::time::Instant;

const N_WORKERS: i32 = 4; // スレッド数
const LOOP_MAX: usize = 100_000; // 最大実行回数 ※スレッド数（N_WORKERS）で割り切れる値を設定する。
// const LOOP_MAX: usize = 1000; // 最大実行回数

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
#[derive (Clone, Copy, Debug)]
struct BlanceSim {
    init_money: i32,
}

impl SimBase for BlanceSim {
    fn is_occure(&mut self, rng: &mut rand::prelude::ThreadRng) -> bool {
        let is_occure: bool = rng.gen_bool(0.8); // 出費が発生するかどうか
        if is_occure {
            let mut saving = Saving::new(self.init_money);
            let expense = rng.gen_range(90..=99); // 出費の額
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
#[derive (Clone, Copy, Debug)]
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
            for _m_cnt in 1..=12 { // 1年==12ヶ月
                let income: i32 = rng.gen_range(26..=45); // 収入の幅
                let expense: i32 = rng.gen_range(25..=35); // 支出の幅
                let balance = income - expense;
                self.deposit_amount += balance; 
                // println!("経過年:{} 月: {} 収入: {} 支出: {} 累積: {}", _y_cnt, _m_cnt, income, expense, self.deposit_amount);
            }
        }
        if self.deposit_amount > 2000 { return true }
        false
    }
}

enum ErrorCode {
    Code1,
    Code2,
    Code3,
}

impl ErrorCode {
    fn output(&self) {
        match &self {
            ErrorCode::Code1 => {
                println!("パラメータを指定してください！！！");
                println!("./sim_mc シミュレーションNo 並列処理有無（0:無し、1:有り）");
            },
            ErrorCode::Code2 => {
                println!("シミュレーションNoには数値を入力してください！！！");
            },
            ErrorCode::Code3 => {
                println!("並列処理有無には、0又は1を入力して下さい。0:無し、1:有り");
            },
        }
    }
}

enum Conf
{
    Sim1 { thread_flag: bool },
    Sim2 { thread_flag: bool },
    // Ng,
}

impl Conf
{
    fn new(args: &Vec<String>) -> Result<Conf, ErrorCode>
    {
        if args.len() != 3 {
            Err(ErrorCode::Code1)
        } else {
            let thread_flag: bool = match args[2].parse::<i32>() {
                Ok(v) if v == 1 => Ok(true),
                Ok(v) if v == 0 => Ok(false),
                Err(_) => Err(ErrorCode::Code3),
                _ => Err(ErrorCode::Code3),
            }?; // 変換できなければ即時リターン（"?"をつけるとErrの場合、即時リターンできる）

            let conf: Conf = match args[1].parse::<i32>() {
                Ok(v) if v == 1 => Ok(Conf::Sim1 { thread_flag }),
                Ok(v) if v == 2 => Ok(Conf::Sim2 { thread_flag }),
                Err(_) => Err(ErrorCode::Code2),
                _ => Err(ErrorCode::Code2),
            }?; 

            return Ok(conf);
        }
    }

    fn run(&self) -> usize {
        match &self {
            Conf::Sim1 { thread_flag: tf } => {
                let blance_sim = BlanceSim { init_money : 95 };
                if *tf {
                    return thread_process(blance_sim);
                } else {
                    return one_thread(blance_sim);
                }
            },
            Conf::Sim2 { thread_flag: tf } => {
                let life_depo = LifeDepositSim {init_deposit_amount: 95, deposit_amount: 0 };
                if *tf {
                    return thread_process(life_depo);
                } else {
                    return one_thread(life_depo);
                }
            },
        }

    }
}

/// 使用例
/// ./sim_mc シミュレーションNo 並列処理有無（0:無し、1:有り）
fn main() {    
    // コマンドライン引数を得る
    let args: Vec<String> = env::args().collect();
    let conf: Conf = Conf::new(&args).unwrap_or_else(|err| {
        err.output();
        std::process::exit(1);
    });
    let start_time = Instant::now();
    run(conf);
    show_time(start_time);
}

fn run(conf: Conf) {
    let occure_cnt = conf.run();
    println!("確率: {}, 発生回数: {}, 実行回数: {}"
        , occure_cnt as f32 /LOOP_MAX as f32, occure_cnt, LOOP_MAX);
}

fn one_thread <T> (mut sim_model: T) -> usize
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

fn thread_process <T: 'static> (sim_model: T) -> usize
    where T: SimBase + std::marker::Send + Copy
{
    let thread_loop_max = LOOP_MAX/(N_WORKERS as usize);
    let (tx, rx) = mpsc::channel();
    for _num in 1..=N_WORKERS {
        // ここの tx は for が回るたびにスコープから抜ける => drop する
        let tx = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            let occure_cnt = run_thread(sim_model, thread_loop_max);
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

fn show_time(start_time: Instant) {
    let elapsed = start_time.elapsed();
    println!("実行時間: {:?}", elapsed);
}