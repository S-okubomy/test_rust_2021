use proconio::{ input, fastout };
use std::collections::VecDeque;

fn main() {
    is_it_connected_stack();
}

/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_am
#[allow(dead_code)]
#[allow(non_snake_case)]
#[fastout]
fn is_it_connected_stack() {
    input! {
        N: usize, M: usize,
        AB: [(usize, usize); M],
    }

    let mut neighber_nodes: Vec<Vec<usize>> = vec![vec![]; N+1];
    for ab in AB {
        let (a, b) = ab;
        neighber_nodes[a].push(b);
        neighber_nodes[b].push(a);
    }
    
    let mut visited: Vec<bool> = vec![false; N+1];
    visited[1] = true;
    
    let mut deque: VecDeque<usize> = VecDeque::new();
    deque.push_back(1);

    while deque.len() > 0 {
        // println!("deque1: {:?}", deque);
        let len = deque.len();
        let pos = deque[len-1]; // 先頭(末尾)の要素確認
        deque.pop_back(); // 先頭(末尾)を取り出す
        for neighber_n in &neighber_nodes[pos] {
            if !visited[*neighber_n] {
                visited[*neighber_n] = true;
                deque.push_back(*neighber_n); // 先頭(末尾)に追加
                // println!("deque2: {:?}", deque);
            }
        }
    }

    for v in visited.iter().skip(1) {
        if !v {
            println!("The graph is not connected.");
            return;
        }
    }
    println!("The graph is connected.");
}


/// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_am
#[allow(dead_code)]
#[allow(non_snake_case)]
#[fastout]
fn is_it_connected_rec() {
    input! {
        N: usize, M: usize,
        AB: [(usize, usize); M],
    }

    // let mut neighber_nodes: VecDeque<usize> = VecDeque::new();
    let mut neighber_nodes: Vec<Vec<usize>> = vec![vec![]; N+1];
    for ab in AB {
        let (a, b) = ab;
        neighber_nodes[a].push(b);
        neighber_nodes[b].push(a);
    }
    let mut visited: Vec<bool> = vec![false; N+1]; // false: 通ってない、true: 通った
    dfs(&mut visited, &neighber_nodes, 1);

    for v in visited.iter().skip(1) {
        if !v {
            println!("The graph is not connected.");
            return;
        } 
    }
    println!("The graph is connected.");
}

fn dfs(visited: &mut Vec<bool>, neighber_nodes: &Vec<Vec<usize>>, pos: usize) -> () {
    visited[pos] = true;
    for neighber_n in &neighber_nodes[pos] {
        if !visited[*neighber_n] { // 頂点通っていなければ、通る
            dfs(visited, neighber_nodes, *neighber_n)
        }
    }
}