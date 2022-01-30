use futures::executor::block_on;
use futures::join;

async fn hello_world() {
    println!("hello world")
}

async fn hello_world2() {
    println!("hello world2")
}

async fn run() {
    join!(hello_world(), hello_world2());
}

fn main() {
    block_on(run());

    let mut a = 123;	// ミュータブルな変数aを定義
    let p = &mut a;	// ミュータブルな参照pを定義
    *p = 456;		// 参照先の値を456に書き換える
    println!("{}", a);	// => 456
}