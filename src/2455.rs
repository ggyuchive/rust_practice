use std::io;
use std::str::FromStr;
use std::cmp::max;
fn main(){
    let mut ans = 0;
    let mut tmp = 0;
    for i in 0..4 {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("");
        let arr: Vec<_> = input.split_whitespace().filter_map(|s| i32::from_str(s).ok()).collect();
        tmp = tmp - arr[0] + arr[1];
        ans = max(ans, tmp);
    }
    print!("{}", ans);
}