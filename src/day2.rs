use std::io::prelude::*;
use std::fs::File;

fn min_max_diff(line: &str) -> u32 {
    let min_max = line
        .split_whitespace()
        .map(|digit| digit.parse::<u32>().unwrap())
        .fold(None, |acc, num| if let Some((min, max)) = acc {
            Some((u32::min(min, num), u32::max(max, num)))
        } else { 
            Some((num, num))
        });
    
    match min_max {
        Some((min, max)) => max - min,
        _ => 0
    }
}

fn evenly_divisible(line: &str) -> u32 {
    let mut digits: Vec<u32> = line
        .split_whitespace()
        .map(|digit| digit.parse::<u32>().unwrap())
        .collect();

    digits.sort();
    
    for (i, &d) in digits.iter().enumerate() {
        for &num in digits[i + 1..].iter() {
            if num % d == 0 {
                return num / d;
            }
        }
    }

    0

}

pub fn solve() {
    let mut f = File::open("./inputs/day2.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    let sum: u32 = contents
        .lines()
        // .map(min_max_diff)
        .map(evenly_divisible)
        .sum();
;
    
    println!("{:?}", sum);
}
