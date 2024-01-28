use std::io;
use std::str::FromStr;

pub fn abs(x: i64) -> i64 {
    let ret = if x >= 0 {x} else {-x};
    ret
}
fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    /*string to vector i64 (split w/ whitespace)*/
    let arr: Vec<_> = input.split_whitespace().filter_map(|s| i64::from_str(s).ok()).collect();
    print!("{}", abs(arr[0]-arr[1]));
}
