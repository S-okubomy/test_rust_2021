use std::{env, path, fs};
use regex::Regex;

// 使用例（正規表現）
// ./my_grep1 "abc[\s\S]+*def" /home/okb/デスクトップ/bk/

fn main() {
    // コマンドライン引数を得る --- (*1)
    let args: Vec<String> = env::args().collect();
    // カレントディレクトリを指定

    if args.len() < 3 {
        println!("パラメータを指定してください！！！");
        println!("./my_grep 検索文字 パス");
        return;
    }

    let serch_word = &args[1];
    let target_dir = &args[2];
    
    // PathBufに変換
    let target = path::PathBuf::from(target_dir);
    println!("{}", target_dir);
    tree(&serch_word, &target, 0);
}

// 再帰的にファイル一覧を表示 --- (*2)
fn tree(serch_word: &str, target: &path::PathBuf, level: isize) {
    // ファイル一覧を取得 --- (*3)
    let files = target.read_dir().expect("存在しないパス");
    // 繰り返し表示
    for ent in files {
        // PathBufを取得 --- (*4)
        let path = ent.unwrap().path();
        // level分だけインデント --- (*5)
        for _ in 1..=level {
            print!("|   ");
        }
        // ファイル名を得る --- (*6)
        let fname = path.file_name().unwrap()
            .to_string_lossy();
        // ディレクトリなら再帰的に表示 --- (*7)
        if path.is_dir() {
            println!("|-- <{}>", fname);
            tree(&serch_word, &path, level+1);
            continue;
        }
        // ファイル名を表示 --- (*8)
        println!("|-- {}", fname);

        // テキストファイルを読む --- (*3)
        let text: String = fs::read_to_string(&path).unwrap_or("読み込み失敗".to_string());


        // 西暦、月、及び日をそれぞれ抽出する
        // let re = Regex::new(r"abc[\s\S]+*def").unwrap();
        let re = Regex::new(&serch_word).unwrap();
        for caps in re.captures_iter(&text) {
            // println!("fn:{}, res:{}", fname, &caps[0]);
            println!("file name: {}", fname);
            println!("{}", &caps[0]);
            // println!("{}", &caps[0]);
        }
        // let caps = re.captures(&text).unwrap();
        // println!("{}-{}-{}", &caps[1], &caps[2], &caps[3]);


        // // 一行ごとに区切る --- (*4)
        // let lines = text.split('\n');

        // // 繰り返し加算 --- (*5)
        // for line in lines {
        //     // println!("fn:{}, res:{}", fname, line);
        //    // 行に単語が含まれるか検索 --- (*7)
        //    if line.find(serch_word) == None { continue; }
        //    println!("fn:{}, res:{}", fname, line);
        // }
    }
}