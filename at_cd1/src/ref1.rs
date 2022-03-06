fn main() {
    let numbers = read_numbers();
    if let [n] = numbers[..] {
        solve(n);
    }
}

fn solve(n: u64) {
    let f = fac(n);
    println!("{}", f);
}

fn fac(n: u64) -> u64 {
    fac1(n, 1)
}

fn fac1(n: u64, acc: u64) -> u64 {
    if n == 0 {
        acc
    } else {
        fac1(n - 1, n * acc)
    }
}

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line
}

fn read_numbers() -> Vec<u64> {
    read_line()
        .trim()
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect()
}

fn yesno(b: bool) -> String {
    if b {
        String::from("Yes")
    } else {
        String::from("No")
    }
}