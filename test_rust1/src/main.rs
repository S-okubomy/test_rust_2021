use futures::executor::block_on;
use futures::join;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};

async fn hello_world() {
    println!("hello world")
}

async fn hello_world2() {
    println!("hello world2")
}

async fn run() {
    join!(hello_world(), hello_world2());
}

fn some_method(arg: &str) -> &str {
    arg
}

fn some_method2<'a>(arg1: &'a str, arg2: &str) -> &'a str { // missing lifetime specifier
    arg1
}

fn some_method3<'a>(arg1: &str, arg2: &'a str) -> &'a str { // missing lifetime specifier
    arg2
}

fn some_method4<'a>(arg1: &'a str, arg2: &'a str) -> &'a str { // missing lifetime specifier
    arg1
}


fn maybe_remove_prefix<'a>(orig: &'a [i32], prefix: &[i32]) -> &'a [i32] {
    if orig.starts_with(prefix) {
      return orig.split_at(prefix.len()).1;
    } else {
      return orig;
    }
}


fn main() {
    block_on(run());

    let mut a = 123;	// ミュータブルな変数aを定義
    let p = &mut a;	// ミュータブルな参照pを定義
    *p = 456;		// 参照先の値を456に書き換える
    println!("{}", a);	// => 456


    // 僕をイシュマエルとお呼び。何年か前・・・
    let novel = String::from("Call me Ishmael. Some years ago");
    // let mut first_sentence = novel.split('.')
    //     .next()
    //     .expect("Could not find a '.'");  // '.'が見つかりませんでした

    let mut iter_t = novel.split('.');
    
    println!("test {}", iter_t.next().unwrap());
    println!("test2 {}", iter_t.next().unwrap());
    // println!("test2-1 {}", iter_t.next().unwrap());
    println!("test3 {}", iter_t.next().unwrap_or("見つかりません"));



    let a = [1, 2, 3];
    let suffix;
    {
        let b = [1, 2];
        suffix = maybe_remove_prefix(&a, &b);
        // suffix = maybe_remove_prefix(&b, &a);
        // a = [4, 5, 6];
    }
    println!("{:?}", suffix);  // => [3]

    some_method3("test1", "test2");

    let out: &str;
    let x: &str = "x";
    {
        let y = "y";
        out = some_method4(x, y);
    }
    println!("{}", out);


    let z;
    let x = "foo".to_string();
    {
        let y = "bar".to_string();
        z = some_method6(&x, &y); // `y` does not live long enough
    }
    println!("{}", z);


    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();


    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(1000));
        // こちらがベクタ: {:?}
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    // 値は{}です
    println!("Got: {}", received);




    let counter = Arc::new(Mutex::new(5));
    let mut handles = vec![];

    for thr in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            println!("thr:{}", thr);
            let mut num = counter.lock().unwrap();

            *num += 1;
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());


    let some_u8_value: Option<u8> = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    }
    

}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

fn some_method5<'a>(arg1: &'a String, arg2: &'a String) -> &'a String {
    arg1
}

fn some_method6<'a>(arg1: &'a String, arg2: & String) -> &'a String {
    arg1
}
