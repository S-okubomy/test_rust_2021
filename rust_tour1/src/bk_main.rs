struct SeaCreature {
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}






fn example() -> i32 {
    let x = 42;
    let v = if x < 42 { -1 } else { 1 };
    println!("if より: {}", v);

    let food = "ハンバーガ";
    let result = match food {
        "ホットドッグ" => "ホットドッグです",
        _ => "ホットドッグではありません。",
    };
    println!("食品の識別: {}", result);

    let v = {
        let a = 1;
        let b = 2;
        a + b
    };

    println!("ブロックより: {}", v);

    v + 4
}


fn main() {
    let x = 42;

    match x {
        0 => {
            println!("found zero");
        }
        1 | 2 => {
            println!("found 1 or 2");
        }
        3..=9 => {
            println!("found a number 3 to 9 inclusively")
        }
        matched_num @ 10..=100 => {
            println!("found {} number between 10 to 100", matched_num);
        }
        _ => {
            println!("found something else!");
        }
    }


    let mut x1 = 0;
    let v = loop {
        x1 += 1;
        if x1 == 13 {
            break "13発見";
        }
    };

    println!("loopの戻り知: {}", v);

    println!("関数より: {}", example());


    let s = String::from("Hello world!");
    println!("{} is {} characters long.", s, s.len());



}
