use std::collections::HashMap;

/// TF-IDFの計算
/// https://www.sejuku.net/blog/26420

fn tf(trg: &str, d: &Vec<&str>) -> f64 {
    str_count(trg, d) as f64 / d.len() as f64
}

fn idf(t: &str, docs: &Vec<Vec<&str>>) -> f64 {
    let mut df: f64 = 0.0;
    for doc in docs {
        if doc.contains(&t) {
            df += 1.0_f64;
        }
    }
    println!("確認: {} {} ", docs.len(), df);
    ((docs.len() as f64) / df).ln() + 1.0_f64
    // ((docs.len() as f64) / df).log10() + 1.0_f64
}

fn str_count(trg: &str, d: &Vec<&str>) -> usize {
    let mut letters: HashMap<&str, usize> = HashMap::new();
    for s in d {
        let cnt = letters.entry(s).or_insert(0);
        *cnt += 1;
    }
    *letters.get(trg).unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::*; // モジュールの外側で定義したリソース使用

    const ADD_EPSILON: f64 = 2.2204460492503131E-6_f64;

    #[test]
    fn str_count_test1() {
        let d = vec!["猫", "小さい", "犬", "犬", "可愛い", "可愛い", "犬", "大きい"];
        assert_eq!(str_count("犬", &d), 3);
        assert_eq!(str_count("猫", &d), 1);
        assert_eq!(str_count("ギター", &d), 0);
        assert_eq!(str_count("小さい", &d), 1);
    }

    #[test]
    fn tf_test1() {
        let d = vec!["犬", "可愛い", "犬", "大きい"];
        assert_eq!(tf("可愛い", &d), 0.25);
        assert_eq!(tf("可愛くない", &d), 0.0);
        assert_eq!(tf("大きい", &d), 0.25);
        assert_eq!(tf("小さい", &d), 0.0);
        assert_eq!(tf("犬", &d), 0.5);
        assert_eq!(tf("猫", &d), 0.0);
        assert_eq!(tf("虫", &d), 0.0);
    }

    #[test]
    fn idf_test1() {
        let docs: Vec<Vec<&str>> = vec![
                vec!["犬", "可愛い", "犬", "大きい"],
                vec!["猫", "小さい", "猫", "可愛い", "可愛い"],
                vec!["虫", "小さい", "可愛くない"]
        ];
        let res: f64 = idf("可愛い", &docs);
        let exp: f64 = 1.405465;
        let abs_diff = (exp - res).abs();
        assert!(judge_diff(idf("可愛い", &docs), 1.405465));
        // println!("{}", idf("可愛くない", &docs));
        assert!(judge_diff(idf("可愛くない", &docs), 2.098612));
        assert!(judge_diff(idf("大きい", &docs), 2.098612));
        assert!(judge_diff(idf("小さい", &docs), 1.405465));
        assert!(judge_diff(idf("犬", &docs), 2.098612));
        assert!(judge_diff(idf("猫", &docs), 2.098612));
        assert!(judge_diff(idf("虫", &docs), 2.098612));
    }

    fn judge_diff(res: f64, exp: f64) -> bool {
        let abs_diff = (exp - res).abs();
        abs_diff <= f64::EPSILON + ADD_EPSILON
    }
}