use std::io;
use std::str::FromStr;
use std::cmp::max;

pub fn solve(_n: String, _v1: String, _v2: String) -> i32 {
    let n = _n.trim().parse().expect("");
    let v1: Vec<_> = _v1.split_whitespace().filter_map(|s: &str| i32::from_str(s).ok()).collect();
    let v2: Vec<_> = _v2.split_whitespace().filter_map(|s| i32::from_str(s).ok()).collect();
    let mut arr: Vec<Vec<i32>> = vec![vec![-1; 4]; n]; // n*4 array
    return dp(n, &v1, &v2, 0, 0, &mut arr);
}

pub fn dp(n: usize, v1: &[i32], v2: &[i32], lu: usize, ld: usize, arr: &mut Vec<Vec<i32>>) -> i32 {
    if n == 0 {
        return 0;
    }
    else if arr[n-1][lu*2+ld] != -1 {
        return arr[n-1][lu*2+ld];
    }
    let mut ret = 0;
    if lu == 0 {
        ret = max(ret, v1[0] + dp(n-1, &v1[1..], &v2[1..], 1, 0, arr));
    }
    if ld == 0 {
        ret = max(ret, v2[0] + dp(n-1, &v1[1..], &v2[1..], 0, 1, arr));
    }
    ret = max(ret, dp(n-1, &v1[1..], &v2[1..], 0, 0, arr));
    arr[n-1][lu*2+ld] = ret;
    return ret
}
fn main(){
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("");

    let tc: i32 = input.trim().parse().expect("");
    let mut cnt = 0;
    loop {
        cnt += 1;
        let mut n: String = String::new();
        let mut v1: String = String::new();
        let mut v2: String = String::new();
        io::stdin().read_line(&mut n).expect("");
        io::stdin().read_line(&mut v1).expect("");
        io::stdin().read_line(&mut v2).expect("");
        println!("{}", solve(n, v1, v2));
        if cnt == tc {
            break;
        }
    }
}