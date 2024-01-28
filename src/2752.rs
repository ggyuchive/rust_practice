use std::io;
use std::str::FromStr;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    /*Must declare mut to change variable*/
    let mut arr: Vec<_> = input.split_whitespace().filter_map(|s| u32::from_str(s).ok()).collect();
    for i in 0..3 {
        for j in i+1..3 {
            if arr[i] > arr[j] {
                let tmp = arr[i];
                arr[i] = arr[j];
                arr[j] = tmp;
            }
        }
    }
    print!("{} {} {}", arr[0], arr[1], arr[2]);
}
