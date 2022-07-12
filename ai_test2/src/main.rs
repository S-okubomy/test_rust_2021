use lindera::tokenizer::Tokenizer;
use lindera::LinderaResult;

use std::error::Error;
use std::process;
use std::fs::File;
use std::{env, path, fs, io};

mod nlp;
use nlp::tf_idf;


enum ExecMode {
    Learn,
    Predict { que_sentence: String },
}

impl ExecMode {
    fn new(args: &Vec<String>) -> Result<ExecMode, String> {
        if args.len() < 2 {
            return Err("引数を指定してください。学習: l、予測: p".to_string())
        }
        let mode = args[1].as_str();
        match mode {
            "l" => {
                Ok(ExecMode::Learn)
            },
            "p" => {
                if args.len() == 3 {
                    Ok(ExecMode::Predict { que_sentence: args[2].to_string() })
                } else {
                    Err("予測時は、p入力後に質問文を入力してください。".to_string())
                }
            },
            _ => {
                Err("学習: l、予測: p を指定してください。".to_string())
            }
        }
    }
}

// trait 


/// 使用例
/// 学習時: ./ai_test l
///          time cargo run l
/// 予測時: ./ai_test p お店でギター演奏できますか？
///         time cargo run p お店でギター演奏できますか？
fn main() -> LinderaResult<()> {
    // コマンドライン引数を得る
    let args: Vec<String> = env::args().collect();
    let exec_mode: ExecMode = ExecMode::new(&args).unwrap_or_else(|err| {
        println!("error running read: {}", err);
        std::process::exit(1);
    });
    run(exec_mode);
    Ok(())
}

fn run(mode: ExecMode) {
    match mode {
        ExecMode::Learn => {
            learn();
        },
        ExecMode::Predict { que_sentence } => {
            predict(que_sentence);
        },
        _ => (),
    }
}

fn learn() {
    let qa_data: QaData = read_csv().unwrap_or_else(|err| {
        println!("error running read: {}", err);
        std::process::exit(1);
    });

    let mut docs: Vec<Vec<String>> = Vec::new();
    for input_qa in qa_data.que_vec {
        let doc_vec: Vec<String> = get_tokenizer(input_qa).unwrap();
        docs.push(doc_vec);
    }

    out_csv_word(&docs);

    // TODO 1. 教師データ読み込んで、tf-idfベクトル作成
    let tf_idf_res = tf_idf::TfIdf::get_tf_idf(&docs);
    // 学習済みモデル出力
    out_csv(tf_idf_res);
}

fn predict(que_sentence: String) {
    let qa_data: QaData = read_csv().unwrap_or_else(|err| {
        println!("error running read: {}", err);
        std::process::exit(1);
    });

    let docs: Vec<Vec<String>> = read_word_list_csv().unwrap_or_else(|err| {
        println!("error running read: {}", err);
        std::process::exit(1);
    });

    let tfidf: tf_idf::TfIdf = read_model_csv().unwrap();
    let trg: Vec<String> = get_tokenizer(que_sentence.to_owned()).unwrap();
    let (id, cos_val) = tf_idf::TfIdf::predict(tfidf, &docs, &trg);
    println!("id: {}", id);
    println!("cos類似度: {}", cos_val);
    println!("質問文: {}", que_sentence.to_owned());
    println!("類似の質問文: {}", qa_data.que_vec[id]);
    println!("回答: {}", qa_data.ans_vec[id]);
}


fn get_tokenizer(doc: String) -> LinderaResult<Vec<String>> {
    // create tokenizer
    let tokenizer = Tokenizer::new()?;

    // tokenize the text
    let tokens = tokenizer.tokenize(doc.as_str())?;

    // output the tokens
    let mut docs: Vec<String> = Vec::new();
    for token in tokens {
        docs.push(token.text.to_string());
        // println!("{}", token.text);
    }

    Ok(docs)
}

#[derive(Debug)]
struct QaData {
    que_vec: Vec<String>,
    ans_vec: Vec<String>,
}

impl QaData {
    fn new(que_vec: Vec<String>, ans_vec: Vec<String>) -> Self {
        Self { que_vec, ans_vec }
    }
}

fn read_csv() -> Result<QaData, Box<dyn Error>> {
    let csv_file_path = "input/study_qa1.csv";
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false) // ヘッダーが無い事を明示的に設定
        .from_path(csv_file_path)?;

    let mut que_vec: Vec<String> = Vec::new();
    let mut ans_vec: Vec<String> = Vec::new();
    for result in rdr.records() {
        let record = result?;
        que_vec.push(record[3].to_string());
        ans_vec.push(record[2].to_string())
    }
    Ok(QaData { que_vec, ans_vec })
}

fn read_word_list_csv() -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let csv_file_path = "output/word_list.csv";
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false) // ヘッダーが無い事を明示的に設定
        .flexible(true) // 可変長で読み込み
        .from_path(csv_file_path)?;

    let mut word_v_v: Vec<Vec<String>> = Vec::new();
    for (index, result) in rdr.records().enumerate() { // ヘッダーは除く
        let record = result?;
        word_v_v.push(vec![]);
        for col in &record {
            word_v_v[index].push(col.to_string());
        }
    }

    Ok(word_v_v)
}

fn read_model_csv() -> Result<tf_idf::TfIdf, Box<dyn Error>> {
    let model_csv_file_path = "output/model_qa1.csv";
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false) // ヘッダーが無い事を明示的に設定
        .from_path(model_csv_file_path)?;

    let mut rec_v_v: Vec<Vec<String>> = Vec::new();
    for (index, result) in rdr.records().enumerate() { // ヘッダーは除く
        let record = result?;
        rec_v_v.push(vec![]);
        for col in &record {
            rec_v_v[index].push(col.to_string());
        }
    }
    let word_vec: Vec<String> = (rec_v_v[0][1..]).to_vec(); // "id"の文字以降を格納
    let mut tf_idf_vec: Vec<Vec<f64>> = Vec::new();
    for (index, rec_v) in rec_v_v.iter().skip(1).enumerate() { // ヘッダーは除く
        tf_idf_vec.push(vec![]);
        for tf_idf in rec_v {
            let tf_idf_val: f64 = tf_idf.parse::<f64>().unwrap();
            tf_idf_vec[index].push(tf_idf_val);
        }
    }

    let tfidf: tf_idf::TfIdf = tf_idf::TfIdf {
        word_vec,
        tf_idf_vec
    };

    Ok(tfidf)
}

/// csv出力
/// https://qiita.com/algebroid/items/c456d4ec555ae04c7f92
fn out_csv(tf_idf_res: tf_idf::TfIdf) -> Result<(), Box<dyn Error>> {
    let csv_file_out_path = "output/model_qa1.csv";
    let mut wtr = csv::WriterBuilder::new()
        .quote_style(csv::QuoteStyle::Always)
        .from_path(csv_file_out_path)?;

    // let mut wtr = csv::Writer::from_path(csv_file_out_path)?;

    let mut w_vec = vec!["id"];
    let mut w_add_vec: Vec<&str> = tf_idf_res.word_vec.iter().map(|s| s.as_str()).collect();
    w_vec.append(&mut w_add_vec);
    wtr.write_record(&w_vec)?;

    for (index, tf_idf_vec) in tf_idf_res.tf_idf_vec.iter().enumerate() {
        let mut s_vec: Vec<String> = vec![index.to_string()];
        let mut s_add_vec: Vec<String> = tf_idf_vec.iter().map(|s| s.to_string()).collect();
        s_vec.append(&mut s_add_vec);
        wtr.write_record(s_vec)?;
    }

    wtr.flush()?;
    Ok(())
}

fn out_csv_word(docs: &Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    let csv_file_out_path = "output/word_list.csv";
    let mut wtr = csv::WriterBuilder::new()
        .quote_style(csv::QuoteStyle::Always)
        .flexible(true) // 可変長で書き込み
        .from_path(csv_file_out_path)?;

    for doc in docs {
        let s_vec: Vec<String> = doc.iter().map(|s| s.to_string()).collect();
        wtr.write_record(s_vec)?;
    }

    wtr.flush()?;
    Ok(())
}