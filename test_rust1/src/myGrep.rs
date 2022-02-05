use std::{env, path, fs};

fn main() {
    // コマンドライン引数を得る --- (*1)
    let args: Vec<String> = env::args().collect();
    // カレントディレクトリを指定
    let mut target_dir = ".";
    let mut serch_word = "";
    if args.len() >= 2 { // パスを指定した場合
        serch_word = &args[1];
        target_dir = &args[2];
    }
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
        // 一行ごとに区切る --- (*4)
        let lines = text.split('\n');

        // 繰り返し加算 --- (*5)
        for line in lines {
            // println!("fn:{}, res:{}", fname, line);
           // 行に単語が含まれるか検索 --- (*7)
           if line.find(serch_word) == None { continue; }
           println!("fn:{}, res:{}", fname, line);

            // // 数値に変換 --- (*6)
            // let n:f64 = match line.parse() {
            //     Ok(v) => v,
            //     Err(_) => 0.0,
            // };
            // total += n;
        }
    }
}