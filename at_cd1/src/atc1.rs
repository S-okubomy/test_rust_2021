fn read <T> () -> T
    where T: std::str::FromStr
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

/// 実行方法
/// cargo run --bin atc1
/// https://atcoder.jp/contests/abc075/tasks/abc075_b
fn main() {
    let map = read_map();
    for y in 0..map.h {
        for x in 0..map.w {
            print!("{}", map.show_at(x, y));
        }
        print!("\n");
    }
}

fn read_map() -> Map {
    let vec: Vec<u32> = read::<String>().split_whitespace()
        .map(|s| s.trim().parse().ok().unwrap()).collect();

    // println!("{:?}", vec);
    let h = vec[0];
    let w = vec[1];
    let mut f: Vec<bool> = Vec::with_capacity((h * w) as usize);
    for _y in 1..=h {
        let line = read::<String>();
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
    h: u32,
    w: u32,
    map: Vec<bool>,
}

impl Map {
    fn at(&self, x: i32, y: i32) -> Option<bool> {
        if 0 <= x && x < self.w as i32 && 0 <= y && y < self.h as i32 {
            self.map
                .get(x as usize + y as usize * (self.w as usize))
                .map(|&c| c)
        } else {
            None
        }
    }
    fn count_neighbour_incl_self(&self, x: u32, y: u32) -> u8 {
        let mut n: u8 = 0;
        for dx in &[-1, 0, 1] {
            for dy in &[-1, 0, 1] {
                n += self.at(x as i32 + dx, y as i32 + dy).unwrap_or(false) as u8;
            }
        }
        n
    }

    fn show_at(&self, x: u32, y: u32) -> String {
        if self.at(x as i32, y as i32).unwrap_or(false) {
            "#".to_string()
        } else {
            format!("{}", self.count_neighbour_incl_self(x, y))
        }
    }
}


/// テスト用
/// cargo build --bin atc1 && cargo test --bin atc1
#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    use std::process::Output;

    const BIN: &'static str = "./atc1"; // 実行ファイル名

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

    // TODO 標準入力が読み取れないので確認 なぜかproconioを使うとうまくいく
    // #[test]
    // fn test1() {
    //     let input = r#"
    //         3 5
    //         .....
    //         .#.#.
    //         .....        
    //     "#;
    //     let out = output(input);
    //     let exp = r#"
    //         11211
    //         1#2#1
    //         11211
    //     "#;
    //     assert_eq!(out.stdout_str(), exp);
    //     assert!(out.stderr_str().is_empty());
    // }
}