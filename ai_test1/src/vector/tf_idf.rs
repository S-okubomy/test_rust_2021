use std::collections::{ HashSet, HashMap };

/// TF-IDFの計算
/// https://www.sejuku.net/blog/26420
pub fn get_tf_idf(docs: &Vec<Vec<&str>>) -> Vec<Vec<f64>> {
    let mut tmp_words: Vec<String> = Vec::new();
    for doc in docs {
        for w in doc {
            tmp_words.push(w.to_string());
        }
    }
    let mut words: HashSet<String> = tmp_words.into_iter().collect();
    // let mut word_vec: Vec<&str> = tmp_words.into_iter().collect::<HashSet<String>>().iter().map(|s| s.as_str()).collect();
    let mut word_vec: Vec<&str> = words.iter().map(|s| s.as_str()).collect();
    println!("{:?}", &word_vec);
    word_vec.sort();
    println!("{:?}", &word_vec);

    let mut tf_idf_vec: Vec<Vec<f64>> = Vec::new();
    let n: usize = docs.len();
    for i in 0..n {
        tf_idf_vec.push(vec![]);
        let d = &docs[i];
        for word in &word_vec {
            tf_idf_vec[i].push(cal_tf_idf(word, &d, &docs));
        }
    }
    println!("{:?}", tf_idf_vec);
    tf_idf_vec
    
}


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
    ((docs.len() as f64) / df).ln() + 1.0_f64
}

fn cal_tf_idf(t: &str, d: &Vec<&str>, docs: &Vec<Vec<&str>>) -> f64 {
    tf(t, d) * idf(t, docs)
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

        assert!(judge_diff(idf("可愛い", &docs), 1.405465));
        // println!("{}", idf("可愛くない", &docs));
        assert!(judge_diff(idf("可愛くない", &docs), 2.098612));
        assert!(judge_diff(idf("大きい", &docs), 2.098612));
        assert!(judge_diff(idf("小さい", &docs), 1.405465));
        assert!(judge_diff(idf("犬", &docs), 2.098612));
        assert!(judge_diff(idf("猫", &docs), 2.098612));
        assert!(judge_diff(idf("虫", &docs), 2.098612));
    }

    #[test]
    fn cal_tf_idf_test1() {
        let docs: Vec<Vec<&str>> = vec![
                vec!["犬", "可愛い", "犬", "大きい"],
                vec!["猫", "小さい", "猫", "可愛い", "可愛い"],
                vec!["虫", "小さい", "可愛くない"]
        ];
        assert!(judge_diff(cal_tf_idf("可愛い", &docs[0], &docs), 0.351366));
        assert!(judge_diff(cal_tf_idf("大きい", &docs[1], &docs), 0.000000));
        assert!(judge_diff(cal_tf_idf("小さい", &docs[1], &docs), 0.281093));
        assert!(judge_diff(cal_tf_idf("虫", &docs[2], &docs), 0.699537));

    }

    #[test]
    fn get_tf_idf_test1() {
        let docs: Vec<Vec<&str>> = vec![
            vec!["犬", "可愛い", "犬", "大きい"],
            vec!["猫", "小さい", "猫", "可愛い", "可愛い"],
            vec!["虫", "小さい", "可愛くない"]
        ];
        get_tf_idf(&docs);
    }

    fn judge_diff(res: f64, exp: f64) -> bool {
        let abs_diff = (exp - res).abs();
        abs_diff <= f64::EPSILON + ADD_EPSILON
    }
}