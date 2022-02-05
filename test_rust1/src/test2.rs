fn sum(n:i32) -> i32{
    if n <= 0 { return 0; }
    return sum(n-1) + n;
}

fn x2(arg: &mut i32) {
    *arg = *arg +2;
}

fn main() {
    println!("{}", sum(10));

    let mut v = 16;
    x2(&mut v);
    println!("{}", v);


    let s = "苦しむ人にはどの日も悪い日である。";
    // 文字列の置換 --- (*1)
    let s2 = s.replace("苦しむ人", "陽気な人");
    let s3 = s2.replace("悪い日", "宴会");
    // 置換前と置換後を表示 --- (*2)
    println!("置換前: {}\n置換後: {}", s, s3);
}