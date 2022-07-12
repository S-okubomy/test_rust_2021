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
    Predict,
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
                Ok(ExecMode::Predict)
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
/// 予測時: ./ai_test p
fn main() -> LinderaResult<()> {
    // コマンドライン引数を得る
    let args: Vec<String> = env::args().collect();
    let exec_mode: ExecMode = ExecMode::new(&args).unwrap_or_else(|err| {
        println!("error running read: {}", err);
        std::process::exit(1);
    });

    run(exec_mode);

    Ok(())

    // let input_qa_vec: Vec<String> = read_csv().unwrap_or_else(|err| {
    //     println!("error running read: {}", err);
    //     std::process::exit(1);
    // });

    // let mut docs: Vec<Vec<String>> = Vec::new();
    // for input_qa in input_qa_vec {
    //     let doc_vec: Vec<String> = get_tokenizer(input_qa).unwrap();
    //     docs.push(doc_vec);
    // }




    // let docs: Vec<Vec<String>> = vec![
    //     vec!["犬", "可愛い", "犬", "大きい"].iter().map(|s| s.to_string()).collect(),
    //     vec!["猫", "小さい", "猫", "可愛い", "可愛い"].iter().map(|s| s.to_string()).collect(),
    //     vec!["虫", "小さい", "可愛くない"].iter().map(|s| s.to_string()).collect()
    // ];

    // TODO 1. 教師データ読み込んで、tf-idfベクトル作成
    // let tf_idf_res = tf_idf::TfIdf::get_tf_idf(&docs);
    // println!("{:?}", tf_idf_res.word_vec);
    // println!("{:?}", tf_idf_res.tf_idf_vec);

    // TODO 2. tf-idfベクトル書き込み（学習モデル作成）

    // cos類似度
    // https://qiita.com/yonedaco/items/ef6fd0db2773f62b0f72
    // https://w3e.kanazawa-it.ac.jp/math/category/vector/henkan-tex.cgi?target=/math/category/vector/naiseki-wo-fukumu-kihonsiki.html

    // TODO 3. 推論

    // if let Err(err) = read() {
    //     println!("error running read: {}", err);
    //     process::exit(1);
    // }

    // out_csv(tf_idf_res);
}

fn run(mode: ExecMode) {
    match mode {
        ExecMode::Learn => {
            learn();
        },
        ExecMode::Predict => {
            predict();
        },
        _ => (),
    }
}

fn learn() {
    let input_qa_vec: Vec<String> = read_csv().unwrap_or_else(|err| {
        println!("error running read: {}", err);
        std::process::exit(1);
    });

    let mut docs: Vec<Vec<String>> = Vec::new();
    for input_qa in input_qa_vec {
        let doc_vec: Vec<String> = get_tokenizer(input_qa).unwrap();
        docs.push(doc_vec);
    }

    // TODO 1. 教師データ読み込んで、tf-idfベクトル作成
    let tf_idf_res = tf_idf::TfIdf::get_tf_idf(&docs);
    println!("{:?}", tf_idf_res.word_vec);
    println!("{:?}", tf_idf_res.tf_idf_vec);

    // 学習済みモデル出力
    out_csv(tf_idf_res);
}

fn predict() {
    let input_qa_vec: Vec<String> = read_csv().unwrap_or_else(|err| {
        println!("error running read: {}", err);
        std::process::exit(1);
    });

    let mut docs: Vec<Vec<String>> = Vec::new();
    for input_qa in input_qa_vec {
        let doc_vec: Vec<String> = get_tokenizer(input_qa).unwrap();
        docs.push(doc_vec);
    }

    let tfidf: tf_idf::TfIdf = read_model_csv().unwrap();
    println!("{:?}", tfidf);

    // let trg: Vec<String> = vec!["犬", "可愛い", "犬", "大きい"].iter().map(|s| s.to_string()).collect();
    let trg_str: &str = "楽器置いてますか？";
    let trg: Vec<String> = get_tokenizer(trg_str.to_string()).unwrap();
    let (id, cos_val) = tf_idf::TfIdf::predict(tfidf, &docs, &trg);
    println!("{} : {}", id, cos_val);
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

fn _test_read() -> Result<(), Box<dyn Error>> {
    // let mut rdr = csv::Reader::from_reader(io::stdin()); // cargo run show < input/samp.csv
    // ファイルを開く --- (*4)
    
    let csv_file_path = "input/model_qa1.csv";
    let fp = File::open(csv_file_path).unwrap();
    let mut rdr = csv::Reader::from_reader(fp); // cargo run show < input/samp.csv
    for result in rdr.records() {
        let record = result?;
        let title = &record[0];
        let body = &record[1];
        let detail = &record[2];
        println!("title: {}", &title);
        println!("body: {}", &body);
        println!("detail: {}", &detail);
    }
    Ok(())
}

fn read_csv() -> Result<Vec<String>, Box<dyn Error>> {
    // let mut rdr = csv::Reader::from_reader(io::stdin()); // cargo run show < input/samp.csv
    // ファイルを開く --- (*4)
    
    let csv_file_path = "input/study_qa1.csv";
    // let fp = File::open(csv_file_path).unwrap();
    // let mut rdr = csv::Reader::from_reader(fp); // cargo run show < input/samp.csv
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false) // ヘッダーが無い事を明示的に設定
        .from_path(csv_file_path)?;

    let mut input_qa_vec: Vec<String> = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let input_doc = record[3].to_string();
        input_qa_vec.push(input_doc);
        // println!("input: {}", &input_doc);
    }
    Ok(input_qa_vec)
}

// fn read_model_csv() -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
//     let model_csv_file_path = "output/model_qa1.csv";
//     let fp = File::open(model_csv_file_path).unwrap();
//     let mut rdr = csv::Reader::from_reader(fp);
//     let mut tf_idf_vec: Vec<Vec<f64>> = Vec::new();
//     for (index, result) in rdr.records().enumerate() { // ヘッダーは除く
//         let record = result?;
//         // println!("{:?}", record);
//         tf_idf_vec.push(vec![]);
//         for tf_idf_s in &record {
//             let tf_idf_val: f64 = tf_idf_s.parse::<f64>().unwrap();
//             tf_idf_vec[index].push(tf_idf_val);
//         }
//     }
//     Ok(tf_idf_vec)
// }

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