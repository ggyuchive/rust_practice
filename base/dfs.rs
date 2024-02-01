use std::io;
use std::str::FromStr;
use std::fmt::Write;

pub fn linetoVec() -> Vec<usize> {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).expect("");
    let v: Vec<_> = line.split_whitespace().filter_map(|s: &str| usize::from_str(s).ok()).collect();
    v
}

pub fn dfs(edge: &Vec<Vec<usize>>, visited: &mut Vec<i32>, s: usize, cnt: &mut i32) -> () {
    visited[s] = *cnt;
    *cnt += 1;
    for e in &edge[s] {
        if visited[*e] == 0 {
            dfs(edge, visited, *e, cnt);
        }
    }
}

fn main(){
    let v = linetoVec();
    let n = v[0]; let m = v[1]; let s = v[2]-1;
    let mut edge = vec![vec![]; n];
    let mut visited = vec![0; n];
    for _ in 0..m {
        let a = linetoVec();
        edge[a[0]-1].push(a[1]-1);
        edge[a[1]-1].push(a[0]-1);
    }
    for i in 0..n {
        edge[i].sort();
    }
    dfs(&edge, &mut visited, s, &mut 1);
    let mut output = String::new();
    for i in 0..n {
        writeln!(output, "{}", visited[i]).unwrap();
    }
    print!("{output}");
}