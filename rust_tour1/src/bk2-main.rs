//struct BagOfHolding<T> {
//    item: T,
//}


struct BagOfHolding<T> {
    item: Option<T>,
}



//enum Item {
//    Inventory(String),
//    None,
//}

//struct BagOfHolding {
//    item: Item,
//}

struct Foo {
    x: i32,
}

fn do_something_that_might_fail(i:i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("正しい値ではありません"))
    }
}



fn main() -> Result<(), String> {
    let i32_bag = BagOfHolding::<i32> { item: None };
    if i32_bag.item.is_none() {
        println!("バッグ空");
    } else {
       println!("バッグ中身有り");
    }

    let i32_bag = BagOfHolding::<i32> { item: Some(42) };
    if i32_bag.item.is_some() {
        println!("バッグには何かある");
    } else {
        println!("バッグには何もない");
    }

    match i32_bag.item {
        Some(v) => println!("バッグに{}を発見", v),
        None => println!("何も見つけられなかった"),
    }

    let v = do_something_that_might_fail(42).unwrap();
    println!("発見: {}", v);

//    let v = do_something_that_might_fail(1).unwrap();
//    println!("発見: {}", v);

//    let v = do_something_that_might_fail(42)?;
//    println!("発見 {:?}", v);

//    match result {
//        Ok(v) => println!("発見 {}", v),
//        Err(_e) => {
//            return Err(String::from("mainで何か問題おきた"));
//        }
//    }


    let mut i32_vec = Vec::<i32>::new();
    i32_vec.push(1);
    i32_vec.push(2);
    i32_vec.push(3);
    println!("i32は{:?}です", i32_vec);
    
    for i_vec in i32_vec.iter() {
        println!("{}", i_vec);
    }
    
    let mut float_vec = Vec::new();
    float_vec.push(1.3);
    float_vec.push(2.3);
    float_vec.push(3.4);
    println!("floatは{:?}です", float_vec);

    let string_vec = vec![String::from("Hello"), String::from("World")];
    
    for word in string_vec.iter() {
        println!("{}", word);
    }

    let foo_a = Foo { x: 42 };
    let foo_b = Foo { x: 13 };

    println!("{}", foo_a.x);
    println!("{}", foo_b.x);

    Ok(())

}
