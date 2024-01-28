use std::io;
use std::str::FromStr;

/*vector in fn parameter is &[{var type}] */
fn solve(v: &[u8]) -> char {
    let mut cnt: u8 = 0;
    for i in 0..4 {
        if v[i] == 0 {
            cnt+=1;
        }
    }
    if cnt >= 1 {
        /*
         * u8 -> char casting is possible
         * casting: {value} as {type}
         */
        return (('A' as u8)+cnt-1) as char
    }
    else {
        return 'E'
    }
}
fn main(){
    for i in 0..3 {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("");
        let arr: Vec<_> = input.split_whitespace().filter_map(|s| u8::from_str(s).ok()).collect();
        println!("{}",solve(&arr));
    }
}
