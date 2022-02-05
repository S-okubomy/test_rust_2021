use std::{env, path, fs, io};
use regex::Regex;


#[derive(Debug)]
enum PramType {
    IoInput { inp_text: String },
    CmdArgs { file_path: String, file_text: String },
    NoPram,
}

impl PramType {
    fn custom_print(&self) {
        match &*self {
            PramType::IoInput { inp_text: inp } => println!("{}", inp.replace("\n", "[\\n]").replace("\r\n", "[\\r\\n]")),
            PramType::CmdArgs { file_path: f_path, file_text: f_text } => println!("{}: {}", f_path, f_text.replace("\n", "[\\n]").replace("\r\n", "[\\r\\n]")),
            PramType::NoPram => {
                println!("パラメータを指定してください！！！");
                println!("./my_grep 検索文字 パス");
                println!("(例)  ./my_grep1 \".*abc[\\s\\S]*?def\" /home/okb/デスクトップ");
                println!("echo 対象文字 | ./my_grep1 検索文字");
            },
        }
    }
}

// 使用例（正規表現）
// ./my_grep1 "ギ.*abc[\s\S]*?def" /home/okb/デスクトップ
// echo "test abcde" | ./my_grep1 ".*bc.*"

fn main() {
    // コマンドライン引数を得る --- (*1)
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let p_type = PramType::NoPram;
        p_type.custom_print();
        return;
    }

    if args.len() == 2 {
        let text = input_str();
        let serch_word = &args[1];

        for caps in get_text(&serch_word, &text) {
            let p_type = PramType::IoInput { inp_text: caps[0].to_string() };
            p_type.custom_print();
        }
    } else if args.len() == 3 {
        let serch_word = &args[1];
        let target_dir = &args[2];
        
        // PathBufに変換
        let target = path::PathBuf::from(target_dir);
        // println!("{}", target_dir);
        tree(&serch_word, &target, 0);
    } else {
        let p_type = PramType::NoPram;
        p_type.custom_print();
        return;
    }
}

// 再帰的にファイル一覧を表示 --- (*2)
fn tree(serch_word: &str, target: &path::PathBuf, level: isize) {
    // ファイル一覧を取得 --- (*3)
    let files = target.read_dir().expect("存在しないパス");
    // 繰り返し表示
    for ent in files {
        // PathBufを取得 --- (*4)
        let path = ent.unwrap().path();
        if path.is_dir() {
            tree(&serch_word, &path, level+1);
            continue;
        }

        // テキストファイルを読む --- (*3)
        let text: String = fs::read_to_string(&path).unwrap_or("読み込み失敗".to_string());
        let file_path: &str = &path.into_os_string().into_string().unwrap();

        for caps in get_text(&serch_word, &text) {
            let p_type = PramType::CmdArgs { file_path: file_path.to_string(), file_text: caps[0].to_string() };
            p_type.custom_print();
        }
    }
}

fn get_text<'a>(serch_word: &str, text: &'a str) -> std::vec::Vec<regex::Captures<'a>> {
    // 正規表現で抽出する
    let re = Regex::new(&serch_word).unwrap();
    re.captures_iter(text).collect::<Vec<_>>()
}

// 標準入力から文字列を得る --- (*3)
fn input_str() -> String {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("入力エラー");
    s.trim_end().to_string()
}