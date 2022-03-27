use proconio::{ input, fastout };
use std::collections::VecDeque;

fn main() {
    breadth_first_search();
}


/// https://atcoder.jp/contests/math-and-algorithm/tasks/abc007_3
fn breadth_first_search() {
    // TODO
    println!("TODO");
}

/// https://atcoder.jp/contests/math-and-algorithm/tasks/typical90_bz
#[allow(dead_code)]
#[allow(non_snake_case)]
#[fastout]
fn easy_graph_problem() {
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

    let mut ans_cnt = 0;
    for i in 1..=N {
        let neighber_num = neighber_nodes[i].len();
        let mut tmp_cnt = 0;
        for j in 0..neighber_num {
            if i > neighber_nodes[i][j] {
                tmp_cnt += 1;
            }
        }
        if tmp_cnt == 1 {
            ans_cnt += 1;
        }
    }
    println!("{}", ans_cnt);
}

/// https://atcoder.jp/contests/math-and-algorithm/tasks/typical90_bz
#[allow(dead_code)]
#[allow(non_snake_case)]
#[fastout]
fn shortest_path_problem() {
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

    let mut deque: VecDeque<usize> = VecDeque::new();
    deque.push_back(1); // Que末尾に追加
    let mut dist: Vec<isize> = vec![-1; N+1];
    dist[1] = 0;
    while deque.len() > 0 {
        // let pos = deque[0];
        let pos: usize = deque.pop_front().unwrap(); // Que先頭調べる&先頭取り出し
        for neighber_n in &neighber_nodes[pos] {
            if dist[*neighber_n] == -1 {
                dist[*neighber_n] = dist[pos] + 1;
                deque.push_back(*neighber_n); // Que末尾に追加
            }
        }
    }

    for i in 1..=N {
        println!("{}", dist[i]);
    }
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