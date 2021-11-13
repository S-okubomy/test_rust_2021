use std::thread;
use std::collections::HashMap;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();



    let a = [1, 2, 3];
    let doubled: Vec<i32> = a.iter()
                         .map(|&x| x * 2)
                         .collect();

    println!("{:?} {:?}", doubled, a);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!("{:?}", scores);



    // イミュータブルな束縛を作っておく。
    let x = 1;
    // `&値` で参照がとれる。
    let y: &isize = &x;
    // ミュータブルな束縛を作っておく。
    let mut a = 1;
    // `&mut 値`でミュータブルな参照がとれる。値もミュータブルである必要がある。
    let b = &mut a;
    // `*参照 = 値`で代入できる。これは`&mut`型ならいつでも可能。
    *b = 2;
    // bの参照先が書き変わっている。aは一定の条件を満たしている（Copyな）ため参照外しができる。
    println!("{} {}", "テスト",&a); // => 2

        let mut a = 10;           // mutable object
    let a_ref1 = &a;          // reference
    let a_mut_ref1 = &mut a;  // mutable reference
    let a_mut_ref2 = &mut a;  // mutable refernece
    *a_mut_ref2 = 20;         // assign
    println!("{} {}", "テスト", a);        // borrow check!! - OK
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(2 == 2);
        assert_eq!(7 + 2, 4);
        assert_ne!(2 + 2, 5);
    }
}
