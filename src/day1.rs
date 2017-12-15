use std::io::prelude::*;
use std::fs::File;

pub fn solve() {
    let mut f = File::open("./inputs/day1.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    let mut ans: u64 = 0;
    let contents = contents.trim().as_bytes();

    for (i, d) in contents.iter().enumerate() {
        let next_idx = (i + contents.len()/2) % contents.len();
        if *d == contents[next_idx] {
            let num = *d - '0' as u8;
            ans += num as u64;
        }
    }

    println!("Answer is: {}", ans);
}
