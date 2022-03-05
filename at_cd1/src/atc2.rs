use proconio::{ input, fastout };
/// 実行方法
/// cargo run --bin atc2
/// https://atcoder.jp/contests/abc075/tasks/abc075_b
#[fastout]
fn main() {
    input!{
        h: isize,
        w: isize,
        map_str: [String; h],
    }

    let map = read_map(h, w, &map_str);
    for y in 0..map.h {
        for x in 0..map.w {
            print!("{}", map.show_at(x, y));
        }
        print!("\n");
    }
}

fn read_map(h: isize, w: isize, map_str: &[String]) -> Map {
    let mut f: Vec<bool> = Vec::with_capacity((h * w) as usize);
    for line in map_str {
        for c in line.trim().chars() {
            match c {
                '.' => f.push(false),
                '#' => f.push(true),
                _ => (),
            }
        }
    }

    Map {
        h: h,
        w: w,
        map: f,
    }
}

struct Map {
    h: isize,
    w: isize,
    map: Vec<bool>,
}

impl Map {
    fn at(&self, x: isize, y: isize) -> Option<bool> {
        if 0 <= x && x < self.w && 0 <= y && y < self.h {
            self.map
                .get((x + y * (self.w)) as usize)
                .map(|&c| c)
        } else {
            None
        }
    }
    fn count_neighbour_incl_self(&self, x: isize, y: isize) -> u8 {
        let mut n: u8 = 0;
        for dx in &[-1, 0, 1] {
            for dy in &[-1, 0, 1] {
                n += self.at(x + dx, y + dy).unwrap_or(false) as u8;
            }
        }
        n
    }

    fn show_at(&self, x: isize, y: isize) -> String {
        if self.at(x, y).unwrap_or(false) {
            "#".to_string()
        } else {
            format!("{}", self.count_neighbour_incl_self(x, y))
        }
    }
}


/// テスト用
/// cargo build --bin atc2 && cargo test --bin atc2
#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    use std::process::Output;

    const BIN: &'static str = "./atc2"; // 実行ファイル名

    fn output(input_str: &str) -> Output {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(input_str)
            .tee_output()
            .expect_success();
        output
    }

    #[test]
    fn test_other() {
        assert_eq!(1+2, 3);
    }

    #[test]
    fn test1() {
        let input = r#"
            3 5
            .....
            .#.#.
            .....        
        "#;
        let out = output(input);
        let exp: &str = &(r#"
            11211
            1#2#1
            11211
        "#
        .trim().split(" ").map(|s| s.replace(" ", "")).collect::<String>() + "\n");
        assert_eq!(out.stdout_str(), exp );
        assert!(out.stderr_str().is_empty());
    }

    #[test]
    fn test2() {
        let input = r#"
            3 5
            #####
            #####
            #####      
        "#;
        let out = output(input);
        let exp: &str = &(r#"
            #####
            #####
            #####
        "#
        .trim().split(" ").map(|s| s.replace(" ", "")).collect::<String>() + "\n");
        assert_eq!(out.stdout_str(), exp );
        assert!(out.stderr_str().is_empty());
    }

    #[test]
    fn test3() {
        let input = r#"
            6 6
            #####.
            #.#.##
            ####.#
            .#..#.
            #.##..
            #.#...
        "#;
        let out = output(input);
        let exp: &str = &(r#"
            #####3
            #8#7##
            ####5#
            4#65#2
            #5##21
            #4#310   
        "#
        .trim().split(" ").map(|s| s.replace(" ", "")).collect::<String>() + "\n");
        assert_eq!(out.stdout_str(), exp );
        assert!(out.stderr_str().is_empty());
    }
}